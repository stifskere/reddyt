use std::env::var;
use std::future::{ready, Ready};

use actix_web::{FromRequest, HttpRequest, Error as ActixError};
use actix_web::dev::Payload;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;

const EMAIL_KEY: &str = "RYT_ADMIN_EMAIL";
const PASSOWRD_KEY: &str = "RYT_ADMIN_PASSWORD";

/// `OptionalAuth` is an Actix Web extractor
/// that handles optional basic authentication.
///
/// It reads the `Authorization` header and compares
/// it against the credentials stored in environment variables.
/// If the credentials match, the `authenticated` field is set
/// to `true`, indicating that the user has been successfully
/// authenticated.
///
/// In the case of a header parsing failure the authentication
/// attempt will be ignored and the `authenticated` field
/// will be set to false.
pub struct OptionalAuth {
    authenticated: bool
}

impl OptionalAuth {
    /// Constructor function, shortener for
    /// `OptionalAuth { authenticated: true|false }`
    #[inline]
    const fn new(authenticated: bool) -> Self {
        Self {
            authenticated
        }
    }

    /// Returns whether the user is authenticated
    /// or not.
    #[inline]
    pub const fn authenticated(&self) -> bool {
        self.authenticated
    }
}

impl FromRequest for OptionalAuth {
    type Error = ActixError;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let Ok(env_user) = var("PORTFOLIO_USER") else {
            return ready(Ok(OptionalAuth::new(false)));
        };

        let Ok(env_password) = var("PORTFOLIO_PASSWORD") else {
            return ready(Ok(OptionalAuth::new(false)));
        };

        let authorized = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|auth| auth.strip_prefix("Basic "))
            .and_then(|encoded| BASE64_STANDARD.decode(encoded).ok())
            .and_then(|decoded| String::from_utf8(decoded).ok())
            .and_then(|creds| {
                let creds = creds.split_once(':');
                Some((creds?.0.to_string(), creds?.1.to_string()))
            })
            .map(|(user, pass)| user == env_user && pass == env_password)
            .unwrap_or(false);

        ready(Ok(OptionalAuth::new(authorized)))
    }
}
