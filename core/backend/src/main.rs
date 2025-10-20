use actix_web::{main, App, HttpServer};
use thiserror::Error;

use std::io::Error as IoError;

use crate::routes::authentication::authentication_scope;

mod routes;
mod utils;

#[derive(Error, Debug)]
enum AppError {
    #[error("{0:#}")]
    Server(#[from] IoError)
}

#[main]
async fn main() -> Result<(), AppError> {
    HttpServer::new(|| {
        App::new()
            .service(authentication_scope())
    })
        .bind(("0.0.0.0", 8081))?
        .run()
        .await?;

    Ok(())
}
