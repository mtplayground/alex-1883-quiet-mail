use std::{
    env::{self, VarError},
    net::SocketAddr,
};

use crate::error::AppError;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub frontend_dist: String,
    pub database_url: String,
    pub database_max_connections: u32,
    pub auth: AuthConfig,
    pub self_url: Option<String>,
    pub email: Option<EmailConfig>,
    pub bootstrap_account: Option<BootstrapAccountConfig>,
    pub legacy_jwt_secret: Option<String>,
}

#[derive(Clone)]
pub struct AuthConfig {
    pub url: String,
    pub app_token: String,
    pub jwks_url: String,
}

#[derive(Clone)]
pub struct EmailConfig {
    pub url: String,
    pub app_token: String,
}

#[derive(Clone)]
pub struct BootstrapAccountConfig {
    pub email: String,
    pub password: String,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_owned());
        let port = env::var("PORT")
            .ok()
            .map(|value| value.parse::<u16>())
            .transpose()
            .map_err(|source| AppError::Config {
                message: "PORT must be a valid u16".to_owned(),
                detail: Some(source.to_string()),
            })?
            .unwrap_or(8080);
        let frontend_dist =
            env::var("FRONTEND_DIST").unwrap_or_else(|_| "frontend/dist".to_owned());
        let database_url = required_env("DATABASE_URL")?;
        let database_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .map(|value| value.parse::<u32>())
            .transpose()
            .map_err(|source| AppError::Config {
                message: "DATABASE_MAX_CONNECTIONS must be a valid u32".to_owned(),
                detail: Some(source.to_string()),
            })?
            .unwrap_or(5);
        let auth = AuthConfig {
            url: required_env("MCTAI_AUTH_URL")?,
            app_token: required_env("MCTAI_AUTH_APP_TOKEN")?,
            jwks_url: required_env("MCTAI_AUTH_JWKS_URL")?,
        };
        let self_url = optional_env("SELF_URL")?;
        let email = optional_pair("MCTAI_EMAIL_URL", "MCTAI_EMAIL_APP_TOKEN")?
            .map(|(url, app_token)| EmailConfig { url, app_token });
        let bootstrap_account = optional_pair("SINGLE_ACCOUNT_EMAIL", "SINGLE_ACCOUNT_PASSWORD")?
            .map(|(email, password)| BootstrapAccountConfig { email, password });
        let legacy_jwt_secret = optional_env("JWT_SECRET")?;

        Ok(Self {
            host,
            port,
            frontend_dist,
            database_url,
            database_max_connections,
            auth,
            self_url,
            email,
            bootstrap_account,
            legacy_jwt_secret,
        })
    }

    pub fn socket_addr(&self) -> Result<SocketAddr, AppError> {
        format!("{}:{}", self.host, self.port)
            .parse::<SocketAddr>()
            .map_err(|source| AppError::Config {
                message: "HOST and PORT must form a valid socket address".to_owned(),
                detail: Some(source.to_string()),
            })
    }

    pub fn log_startup_summary(&self) {
        tracing::info!(
            host = %self.host,
            port = self.port,
            frontend_dist = %self.frontend_dist,
            database_max_connections = self.database_max_connections,
            auth_url = %self.auth.url,
            auth_jwks_url = %self.auth.jwks_url,
            auth_app_token_configured = !self.auth.app_token.is_empty(),
            self_url_configured = self.self_url.as_ref().is_some_and(|value| !value.is_empty()),
            email_configured = self
                .email
                .as_ref()
                .is_some_and(|email| !email.url.is_empty() && !email.app_token.is_empty()),
            bootstrap_account_configured = self.bootstrap_account.as_ref().is_some_and(
                |account| !account.email.is_empty() && !account.password.is_empty()
            ),
            legacy_jwt_secret_configured = self
                .legacy_jwt_secret
                .as_ref()
                .is_some_and(|value| !value.is_empty()),
            "configuration loaded"
        );
    }
}

fn required_env(key: &'static str) -> Result<String, AppError> {
    match env::var(key) {
        Ok(value) if !value.trim().is_empty() => Ok(value),
        Ok(_) | Err(VarError::NotPresent) => Err(AppError::Config {
            message: format!("{key} must be set"),
            detail: None,
        }),
        Err(VarError::NotUnicode(_)) => Err(AppError::Config {
            message: format!("{key} must be valid unicode"),
            detail: None,
        }),
    }
}

fn optional_env(key: &'static str) -> Result<Option<String>, AppError> {
    match env::var(key) {
        Ok(value) if !value.trim().is_empty() => Ok(Some(value)),
        Ok(_) | Err(VarError::NotPresent) => Ok(None),
        Err(VarError::NotUnicode(_)) => Err(AppError::Config {
            message: format!("{key} must be valid unicode"),
            detail: None,
        }),
    }
}

fn optional_pair(
    left_key: &'static str,
    right_key: &'static str,
) -> Result<Option<(String, String)>, AppError> {
    match (optional_env(left_key)?, optional_env(right_key)?) {
        (Some(left), Some(right)) => Ok(Some((left, right))),
        (None, None) => Ok(None),
        _ => Err(AppError::Config {
            message: format!("{left_key} and {right_key} must be set together"),
            detail: None,
        }),
    }
}
