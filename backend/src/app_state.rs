use crate::{auth::AuthService, config::Config, db::Database, email::OutboundEmailService};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub auth: AuthService,
    #[allow(dead_code)]
    pub email: OutboundEmailService,
}

impl AppState {
    pub fn new(config: &Config, database: Database) -> Self {
        Self {
            database,
            auth: AuthService::from_config(config),
            email: OutboundEmailService::from_config(config),
        }
    }
}
