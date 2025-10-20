use std::sync::Arc;

use thiserror::Error;
use envconfig::{Envconfig, Error as EnvconfigError};

use crate::utils::application::configuration::ReddytConfig;

#[derive(Error, Debug)]
pub enum AppContextError {
    #[error("Error while loading coniguration, {0:#}")]
    Config(#[from] EnvconfigError)
}

#[derive(Clone, Debug)]
pub struct AppContext {
    config: Arc<ReddytConfig>
}

impl AppContext {
    pub fn new() -> Result<Self, AppContextError> {
        Ok(Self {
            config: Arc::new(ReddytConfig::init_from_env()?)
        })
    }

    #[inline]
    pub fn config(&self) -> &ReddytConfig {
        &self.config
    }
}
