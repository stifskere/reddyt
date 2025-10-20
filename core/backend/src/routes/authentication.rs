use actix_failwrap::{proof_route, ErrorResponse};
use actix_web::{HttpResponse, Scope};
use actix_web::web::scope;
use thiserror::Error;

use crate::utils::extractors::authentication::OptionalAuth;

#[derive(ErrorResponse, Error, Debug)]
enum AuthenticationRequestError {
    #[error("Invalid or not provided credentials.")]
    #[status_code(401)]
    Unauthorized
}

pub fn authentication_scope() -> Scope {
    scope("/authentication")
        .service(login_route)
}


#[proof_route("POST /login")]
async fn login_route(auth: OptionalAuth) -> Result<HttpResponse, AuthenticationRequestError> {
    if !auth.is_authenticated() {
        return Err(AuthenticationRequestError::Unauthorized);
    }


    Ok(
        HttpResponse::Ok()
            .finish()
    )
}


