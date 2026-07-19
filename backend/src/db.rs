use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{config::Config, error::AppError};

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../migrations");

#[derive(Debug, Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn connect(config: &Config) -> Result<Self, AppError> {
        let pool = PgPoolOptions::new()
            .max_connections(config.database_max_connections)
            .connect(&config.database_url)
            .await
            .map_err(|source| AppError::Database { source })?;

        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), AppError> {
        MIGRATOR
            .run(&self.pool)
            .await
            .map_err(|source| AppError::Migration { source })
    }

    pub async fn check_ready(&self) -> Result<(), AppError> {
        sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map(|_| ())
            .map_err(|source| AppError::Database { source })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
