use email_address::EmailAddress;
use envconfig::{Envconfig, Error as EnvconfigError};
use thiserror::Error;

/// Holds any errors related to the configuration
/// and application environment.
#[derive(Error, Debug)]
pub enum ReddytConfigError {
    #[error("Couldn't load configuration from the environment, {0:#}")]
    Envconfig(#[from] EnvconfigError),

    #[error("The admin email at RYT_ADMIN_EMAIL is not valid.")]
    InvalidEmail
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
    admin_password: String
}

impl ReddytConfig {
    /// This function loads the environment with
    /// a layer of validation, this is required for
    /// self-hosters to debug any missconfiguration.
    pub fn load_validated() -> Result<Self, ReddytConfigError> {
        let initialized = Self::init_from_env()?;

        // Since we use the admin email for basic authentication
        // it must not contain colons, for future proofing
        // the email can be validated as-is, this way alerts
        // can be sent if needed.
        if !EmailAddress::is_valid(initialized.admin_email()) {
            return Err(ReddytConfigError::InvalidEmail);
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
}
