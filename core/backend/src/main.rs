use actix_web::{web::Path, get, main, web::get, App, HttpResponse, HttpServer};
use thiserror::Error;

use std::io::Error as IoError;

mod utils;

#[derive(Error, Debug)]
enum AppError {
    #[error("{0:#}")]
    Server(#[from] IoError)
}

#[get("/test")]
async fn test() -> HttpResponse {
    HttpResponse::Ok().body("Funciona!!!!")
}

#[main]
async fn main() -> Result<(), AppError> {
    HttpServer::new(|| {
        App::new()
            .service(test)
    })
        .bind(("0.0.0.0", 8081))?
        .run()
        .await?;

    Ok(())
}
