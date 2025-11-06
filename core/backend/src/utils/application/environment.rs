use std::str::FromStr;

use email_address::EmailAddress;
use envconfig::{Envconfig, Error as EnvconfigError};
use thiserror::Error;
use sqlx::postgres::PgConnectOptions;

/// Holds any errors related to the configuration
/// and application environment.
#[derive(Error, Debug)]
pub enum ReddytConfigError {
    #[error("Couldn't load configuration from the environment, {0:#}")]
    Envconfig(#[from] EnvconfigError),

    #[error("The admin email at RYT_ADMIN_EMAIL is not valid.")]
    InvalidEmail,

    #[error("DATABASE_URL doesn't contain a valid postgresql database url.")]
    InvalidPostgresUrl
}

/// The application relevant environment variables.
///
/// **This does not load `.env`, that must be done
/// before loading this structure.**
#[derive(Debug, Envconfig)]
pub struct ReddytConfig {
    #[envconfig(from = "RYT_ADMIN_EMAIL")]
    admin_email: String,

    #[envconfig(from = "RYT_ADMIN_PASSWORD")]
    admin_password: String,

    #[envconfig(from = "DATABASE_URL")]
    database_url: String,

    #[envconfig(from = "DATABASE_MIGRATIONS", default = "./migrations")]
    migrations_path: String,
}

impl ReddytConfig {
    /// This function loads the environment with
    /// a layer of validation, this is required for
    /// self-hosters to debug any missconfiguration.
    ///
    /// The validation errors should be explicitly logged
    /// with `log::error`.
    pub fn load_validated() -> Result<Self, ReddytConfigError> {
        let initialized = Self::init_from_env()?;

        // Since we use the admin email for basic authentication
        // it must not contain colons, for future proofing
        // the email can be validated as-is, this way alerts
        // can be sent if needed.
        if !EmailAddress::is_valid(initialized.admin_email()) {
            log::error!(concat!(
                "The configured email is invalid, ",
                "please re-check the environment variables."
            ));

            return Err(ReddytConfigError::InvalidEmail);
        }

        if PgConnectOptions::from_str(initialized.database_url()).is_err() {
            log::error!(
                "The DATABASE_URL doesn't contain a valid postgresql connection url."
            );
            return Err(ReddytConfigError::InvalidPostgresUrl);
        }

        Ok(initialized)
    }

    /// The application configured email
    /// to access the admin panel.
    #[inline]
    pub fn admin_email(&self) -> &str {
        &self.admin_email
    }

    /// The application configured password
    /// to access the admin panel.
    #[inline]
    pub fn admin_password(&self) -> &str {
        &self.admin_password
    }

    /// The application configured
    /// database url 
    #[inline]
    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    /// The path to db migration files
    #[inline]
    pub fn migrations_path(&self) -> &str {
        &self.migrations_path
    }
}
