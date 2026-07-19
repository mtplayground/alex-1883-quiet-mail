use std::{env, net::SocketAddr};

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub frontend_dist: String,
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

        Ok(Self {
            host,
            port,
            frontend_dist,
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
