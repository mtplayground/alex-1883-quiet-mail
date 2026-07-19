use crate::{auth::AuthService, config::Config, db::Database};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub auth: AuthService,
}

impl AppState {
    pub fn new(config: &Config, database: Database) -> Self {
        Self {
            database,
            auth: AuthService::from_config(config),
        }
    }
}
