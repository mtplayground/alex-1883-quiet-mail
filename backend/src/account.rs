use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use rand_core::OsRng;

use crate::{config::BootstrapAccountConfig, db::Database, error::AppError};

const SINGLE_ACCOUNT_ID: i16 = 1;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Account {
    pub id: i16,
    pub email: String,
    pub password_hash: String,
}

pub async fn bootstrap_single_account(
    database: &Database,
    bootstrap: Option<&BootstrapAccountConfig>,
) -> Result<(), AppError> {
    let Some(bootstrap) = bootstrap else {
        tracing::warn!("single account bootstrap credentials are not configured");
        return Ok(());
    };

    if load_single_account(database).await?.is_some() {
        tracing::info!("single account already exists");
        return Ok(());
    }

    let password_hash = hash_password(&bootstrap.password)?;

    sqlx::query(
        r#"
        INSERT INTO accounts (id, email, password_hash)
        VALUES ($1, $2, $3)
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(SINGLE_ACCOUNT_ID)
    .bind(&bootstrap.email)
    .bind(&password_hash)
    .execute(database.pool())
    .await
    .map_err(|source| AppError::Database { source })?;

    if let Some(account) = load_single_account(database).await? {
        tracing::info!(
            account_id = account.id,
            account_email = %account.email,
            password_hash_configured = !account.password_hash.is_empty(),
            "single account bootstrapped"
        );
    }

    Ok(())
}

async fn load_single_account(database: &Database) -> Result<Option<Account>, AppError> {
    sqlx::query_as::<_, Account>(
        r#"
        SELECT id, email, password_hash
        FROM accounts
        WHERE id = $1
        "#,
    )
    .bind(SINGLE_ACCOUNT_ID)
    .fetch_optional(database.pool())
    .await
    .map_err(|source| AppError::Database { source })
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|source| AppError::PasswordHash {
            detail: source.to_string(),
        })
}
