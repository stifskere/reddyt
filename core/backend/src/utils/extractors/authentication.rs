use std::env::var;
use std::future::{ready, Ready};
use std::sync::OnceLock;

use actix_web::error::ErrorInternalServerError;
use actix_web::{FromRequest, HttpRequest, Error as ActixError};
use actix_web::dev::Payload;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use chrono::{Utc, Duration};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::rand_core::OsError;
use rand::rngs::{OsRng, StdRng};
use rand::distr::{Alphanumeric, SampleString};
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

/// The environment variable key for the email.
const EMAIL_KEY: &str = "RYT_ADMIN_EMAIL";
/// The environment variable key for the passowrd.
const PASSOWRD_KEY: &str = "RYT_ADMIN_PASSWORD";
/// The authentication cookie key
pub const COOKIE_KEY: &str = "authentication";
/// How long until the authentication session expires.
pub const AUTH_EXPIRATION_HOURS: i64 = 3;

/// The claims that the application JWT consists of.
///
/// The email is a filler and the expiration is
/// managed by the jwt crate.
#[derive(Serialize, Deserialize, Debug)]
struct OptionalAuthClaims {
    email: String,
    exp: usize
}

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
    token: Option<String>
}

impl OptionalAuth {
    /// Constructor function, shortener for
    /// `OptionalAuth { authenticated: true|false }`
    #[inline]
    const fn new(token: Option<String>) -> Self {
        Self {
            token
        }
    }

    /// If he user is authenticated this returns the
    /// issued JWT token, otherwise None.
    ///
    /// The provided token is only for use within
    /// this structure.
    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
}

impl FromRequest for OptionalAuth {
    type Error = ActixError;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        /// Client errors are ignored for security,
        /// if there is a missmatch in how the authentication
        /// should be passed, the back-end will act like it
        /// wasn't passed at all.
        macro_rules! ignore_error {
            ($sel:pat = $expr:expr) => {
                let $sel = $expr else {
                    return ready(Ok(OptionalAuth::new(None)));
                };
            };
        }

        ignore_error!(Ok(env_user) = var(EMAIL_KEY));
        ignore_error!(Ok(env_password) = var(PASSOWRD_KEY));

        ignore_error!(
            Some(provided_auth) = req
                .headers()
                .get("Authorization")
                .and_then(|header| header.to_str().ok())
                .map(|header| header.to_string())
                .or_else(|| req.cookie(COOKIE_KEY)
                    .map(|value| value.to_string())
                )
        );

        let Ok(jwt_secret) = get_jwt_secret() else {
            return ready(Err(ErrorInternalServerError(
                "Error while generating JWT secret."
            )));
        };

        if provided_auth.starts_with("Basic ") {
            let authorized = provided_auth
                .strip_prefix("Basic ")
                .and_then(|encoded| BASE64_STANDARD.decode(encoded).ok())
                .and_then(|decoded| String::from_utf8(decoded).ok())
                .and_then(|creds| {
                    let creds = creds.split_once(':');
                    Some((creds?.0.to_string(), creds?.1.to_string()))
                })
                .map(|(user, pass)| user == env_user && pass == env_password)
                .unwrap_or(false);

            if !authorized {
                return ready(Ok(OptionalAuth::new(None)));
            }

            let Some(expiration) = Utc::now()
                .checked_add_signed(Duration::hours(AUTH_EXPIRATION_HOURS))
            else {
                return ready(Err(ErrorInternalServerError(
                    "Error while generating JWT expiration."
                )));
            };

            let jwt_claims = OptionalAuthClaims {
                email: env_user.clone(),
                exp: expiration.timestamp() as usize
            };

            let Ok(jwt_token) = encode(
                &Header::default(),
                &jwt_claims,
                &EncodingKey::from_secret(jwt_secret.as_bytes())
            ) else {
                return ready(Err(ErrorInternalServerError(
                    "Error while signing JWT payload."
                )));
            };

            return ready(Ok(OptionalAuth::new(Some(
                BASE64_STANDARD.encode(jwt_token)
            ))));
        } else if provided_auth.starts_with("Bearer ") {
            ignore_error!(Some(jwt_token) = provided_auth.strip_prefix("Bearer "));
            ignore_error!(
                Some(decoded_jwt_token) = BASE64_STANDARD
                    .decode(jwt_token)
                    .ok()
                    .and_then(|token| String::from_utf8(token).ok())
            );

            let result = decode::<OptionalAuthClaims>(
                &decoded_jwt_token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &Validation::new(Algorithm::HS256)
            );

            return ready(Ok(OptionalAuth::new({
                result
                    .ok()
                    .map(|_| jwt_token.to_string())
            })));
        }


        return ready(Ok(OptionalAuth::new(None)));
    }
}

fn get_jwt_secret() -> Result<&'static String, OsError> {
    static SECRET: OnceLock<String> = OnceLock::new();

    let mut rng = StdRng::try_from_rng(&mut OsRng)?;

    Ok(
        SECRET
            .get_or_init(|| {
                Alphanumeric
                    .sample_string(&mut rng, 32)
            })
    )
}
