use actix_web::web::Data;
use actix_web::{main, App, HttpServer};
use thiserror::Error;

use std::io::Error as IoError;

use crate::routes::authentication::authentication_scope;
use crate::utils::application::context::{AppContext, AppContextError};

mod routes;
mod utils;

/// An application initialization error.
///
/// WARNING: This may come displayed as DEBUG.
#[derive(Error, Debug)]
enum AppError {
    #[error("Error while starting HTTP server, {0:#}")]
    Server(#[from] IoError),

    #[error("Couldn't load App Context, {0:#}")]
    Context(#[from] AppContextError)
}

#[main]
async fn main() -> Result<(), AppError> {
    let context = AppContext::new()?;

    HttpServer::new(move || {
        let context = context.clone();

        App::new()
            .app_data(Data::new(context))
            .service(authentication_scope())
    })
        .bind(("0.0.0.0", 8081))?
        .run()
        .await?;

    Ok(())
}
