use std::sync::Arc;

use thiserror::Error;

use crate::utils::application::configuration::{ReddytConfig, ReddytConfigError};

/// Holds any errors related to the application context
/// i.e database connections, environment...
#[derive(Error, Debug)]
pub enum AppContextError {
    #[error("Error while loading coniguration, {0:#}")]
    Config(#[from] ReddytConfigError)
}

/// The application context, registered as data in the
/// HTTP service, can be accessed by anything else
/// registered in it.
///
/// Holds contextual data such as database connections,
/// environment, syncronization...
#[derive(Clone, Debug)]
pub struct AppContext {
    config: Arc<ReddytConfig>
}

impl AppContext {
    /// Initialize the context with documented
    /// defaults.
    pub fn new() -> Result<Self, AppContextError> {
        Ok(Self {
            config: Arc::new(ReddytConfig::load_validated()?)
        })
    }

    /// The application environment configuration.
    #[inline]
    pub fn config(&self) -> &ReddytConfig {
        &self.config
    }
}
