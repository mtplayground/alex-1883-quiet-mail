use std::{env, net::SocketAddr};

use crate::error::AppError;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub frontend_dist: String,
    pub database_url: String,
    pub database_max_connections: u32,
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
        let database_url = env::var("DATABASE_URL").map_err(|_| AppError::Config {
            message: "DATABASE_URL must be set".to_owned(),
            detail: None,
        })?;
        let database_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .map(|value| value.parse::<u32>())
            .transpose()
            .map_err(|source| AppError::Config {
                message: "DATABASE_MAX_CONNECTIONS must be a valid u32".to_owned(),
                detail: Some(source.to_string()),
            })?
            .unwrap_or(5);

        Ok(Self {
            host,
            port,
            frontend_dist,
            database_url,
            database_max_connections,
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
}
