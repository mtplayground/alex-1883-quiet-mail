use axum::{
    extract::State,
    http::{header::COOKIE, HeaderMap},
    response::Redirect,
    Json,
};
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{app_state::AppState, config::Config, db::Database, error::AppError};

const SESSION_COOKIE_NAME: &str = "mctai_session";

#[derive(Clone)]
pub struct AuthService {
    client: reqwest::Client,
    auth_url: String,
    app_token: String,
    jwks_url: String,
    return_to: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SessionClaims {
    pub sub: String,
    pub email: String,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub aud: serde_json::Value,
    pub iss: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub(crate) struct PersistedUser {
    sub: String,
    email: String,
    name: Option<String>,
    picture_url: Option<String>,
    registered: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct SessionResponse {
    authenticated: bool,
    user: PersistedUser,
}

impl AuthService {
    pub fn from_config(config: &Config) -> Self {
        Self {
            client: reqwest::Client::new(),
            auth_url: config.auth.url.clone(),
            app_token: config.auth.app_token.clone(),
            jwks_url: config.auth.jwks_url.clone(),
            return_to: config
                .self_url
                .as_deref()
                .map(frontend_root)
                .unwrap_or_else(|| "http://localhost:5173/".to_owned()),
        }
    }

    pub fn login_url(&self) -> Result<String, AppError> {
        let mut url = Url::parse(&format!("{}/login", self.auth_url.trim_end_matches('/')))
            .map_err(|source| AppError::Auth {
                detail: source.to_string(),
            })?;

        url.query_pairs_mut()
            .append_pair("app_token", &self.app_token)
            .append_pair("return_to", &self.return_to);

        Ok(url.to_string())
    }

    pub async fn verify_session(&self, token: &str) -> Result<SessionClaims, AppError> {
        let header = decode_header(token).map_err(|source| AppError::Auth {
            detail: source.to_string(),
        })?;
        let kid = header.kid.ok_or_else(|| AppError::Auth {
            detail: "session token missing key id".to_owned(),
        })?;
        let jwks = self.fetch_jwks().await?;
        let jwk = jwks.find(&kid).ok_or_else(|| AppError::Auth {
            detail: "session signing key was not found".to_owned(),
        })?;
        let decoding_key = DecodingKey::from_jwk(jwk).map_err(|source| AppError::Auth {
            detail: source.to_string(),
        })?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[self.app_token.as_str()]);
        validation.set_issuer(&[self.auth_url.as_str()]);

        decode::<SessionClaims>(token, &decoding_key, &validation)
            .map(|token_data| token_data.claims)
            .map_err(|source| AppError::Auth {
                detail: source.to_string(),
            })
    }

    async fn fetch_jwks(&self) -> Result<JwkSet, AppError> {
        self.client
            .get(&self.jwks_url)
            .send()
            .await
            .map_err(|source| AppError::Auth {
                detail: source.to_string(),
            })?
            .error_for_status()
            .map_err(|source| AppError::Auth {
                detail: source.to_string(),
            })?
            .json::<JwkSet>()
            .await
            .map_err(|source| AppError::Auth {
                detail: source.to_string(),
            })
    }
}

pub async fn login(State(state): State<AppState>) -> Result<Redirect, AppError> {
    Ok(Redirect::temporary(&state.auth.login_url()?))
}

pub async fn session(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<SessionResponse>, AppError> {
    let token = session_cookie(&headers).ok_or(AppError::Unauthorized)?;
    let claims = state.auth.verify_session(&token).await?;
    tracing::debug!(
        issuer = %claims.iss,
        expires_at = claims.exp,
        email_verified = claims.email_verified.unwrap_or(false),
        audience_claim_present = !claims.aud.is_null(),
        "verified platform session"
    );
    let user = upsert_user(&state.database, &claims).await?;

    Ok(Json(SessionResponse {
        authenticated: true,
        user,
    }))
}

async fn upsert_user(
    database: &Database,
    claims: &SessionClaims,
) -> Result<PersistedUser, AppError> {
    sqlx::query_as::<_, PersistedUser>(
        r#"
        INSERT INTO users (sub, email, name, picture_url)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (sub) DO UPDATE SET
            email = EXCLUDED.email,
            name = EXCLUDED.name,
            picture_url = EXCLUDED.picture_url,
            last_seen_at = NOW()
        RETURNING sub, email, name, picture_url, (xmax = 0) AS registered
        "#,
    )
    .bind(&claims.sub)
    .bind(&claims.email)
    .bind(&claims.name)
    .bind(&claims.picture)
    .fetch_one(database.pool())
    .await
    .map_err(|source| AppError::Database { source })
}

fn session_cookie(headers: &HeaderMap) -> Option<String> {
    headers
        .get_all(COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .filter_map(|cookie| cookie.trim().split_once('='))
        .find_map(|(name, value)| (name == SESSION_COOKIE_NAME).then(|| value.to_owned()))
}

fn frontend_root(self_url: &str) -> String {
    format!("{}/", self_url.trim_end_matches('/'))
}
