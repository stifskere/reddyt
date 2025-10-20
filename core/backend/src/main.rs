use actix_web::{main, App, HttpServer};
use thiserror::Error;

use std::io::Error as IoError;

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
    })
        .bind(("0.0.0.0", 8081))?
        .run()
        .await?;

    Ok(())
}
