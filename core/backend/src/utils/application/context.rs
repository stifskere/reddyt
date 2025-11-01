use std::sync::{
    Arc,
    OnceLock
};

use sqlx::{
    Pool,
    Postgres,
};
use thiserror::Error;

use crate::utils::application::environment::{ReddytConfig, ReddytConfigError};
use crate::utils::db::{
    init_db_connection,
    DbConnectionError,
};

/// Holds any errors related to the application context
/// i.e database connections, environment...
#[derive(Error, Debug)]
pub enum AppContextError {
    #[error("Error while loading coniguration, {0:#}")]
    Config(#[from] ReddytConfigError),
    
    #[error("Error while connecting to the Database, {0:#}")]
    DataBase(#[from] DbConnectionError),
}

/// The application context, registered as data in the
/// HTTP service, can be accessed by anything else
/// registered in it.
///
/// Holds contextual data such as database connections,
/// environment, syncronization...
#[derive(Clone, Debug)]
pub struct AppContext {
    config: Arc<ReddytConfig>,
    connection_pool: OnceLock<Pool<Postgres>>
}

impl AppContext {
    /// Initialize the context with documented
    /// defaults.
    pub async fn new() -> Result<Self, AppContextError> {
        let config = ReddytConfig::load_validated()?;
        let connection_pool = init_db_connection(config.database_url(), config.migrations_path()).await?;
        Ok(Self {
            config: Arc::new(config),
            connection_pool
        })
    }

    /// The application environment configuration.
    #[inline]
    pub fn config(&self) -> &ReddytConfig {
        &self.config
    }
    
    #[inline]
    /// the application connection pool
    pub async fn get_db_connection(&self) -> Option<&Pool<Postgres>> {
        self.connection_pool.get()
    }
}
