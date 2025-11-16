use sqlx::migrate::{MigrateError, Migrator};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error as SqlxError, Pool, Postgres};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbConnectionError {
    #[error("{0:#}")]
    ConnectionError(#[from] SqlxError),

    #[error("{0:#}")]
    MigrateError(#[from] MigrateError),
}

pub async fn init_db_connection(db_url: &str) -> Result<Pool<Postgres>, DbConnectionError> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    Ok(pool)
}
