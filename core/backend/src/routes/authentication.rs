use actix_failwrap::{proof_route, ErrorResponse};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::Duration;
use actix_web::{HttpResponse, Scope};
use actix_web::web::scope;
use thiserror::Error;

use crate::utils::errors::formatters::json_formatter;
use crate::utils::extractors::authentication::{OptionalAuth, COOKIE_KEY};

#[derive(ErrorResponse, Error, Debug)]
#[transform_response(json_formatter)]
enum AuthenticationRequestError {
    #[error("Invalid or not provided credentials.")]
    #[status_code(401)]
    Unauthorized
}

pub fn authentication_scope() -> Scope {
    scope("/authentication")
        .service(login_route)
        .service(logout_route)
}

#[proof_route("POST /login")]
async fn login_route(auth: OptionalAuth) -> Result<HttpResponse, AuthenticationRequestError> {
    match auth.token() {
        Some(token) => {
            let cookie = Cookie::build(COOKIE_KEY, token)
                .path("/")
                .http_only(true)
                .secure(cfg!(not(debug_assertions)))
                .max_age(Duration::hours(3))
                .finish();

            Ok(
                HttpResponse::NoContent()
                    .cookie(cookie)
                    .finish()
            )
        }

        None => Err(AuthenticationRequestError::Unauthorized)
    }
}

#[proof_route("POST /logout")]
async fn logout_route(auth: OptionalAuth) -> Result<HttpResponse, AuthenticationRequestError> {
    auth.token()
        .map(|_| HttpResponse::NoContent()
            .cookie({
                let mut cookie = Cookie::named(COOKIE_KEY);
                cookie.make_removal();
                cookie
            })
            .finish()
        )
        .ok_or(AuthenticationRequestError::Unauthorized)
}
