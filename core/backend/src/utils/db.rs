use sqlx::migrate::{MigrateError, Migrator};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error as SqlxError, Pool, Postgres};
use std::{path::Path, sync::OnceLock};
use thiserror::Error;

static CONNECTION: OnceLock<Pool<Postgres>> = OnceLock::new();

#[derive(Error, Debug)]
pub enum DbConnectionError {
    #[error("{0:#}")]
    ConnectionError(#[from] SqlxError),

    #[error("{0:#}")]
    MigrateError(#[from] MigrateError),
}

pub async fn get_db_connection<'r>() -> &'r Pool<Postgres> {
    CONNECTION.get().expect("Connection Pool must be initialized first!")
}

pub async fn init_db_conn_pool<'r>(db_url: &str, migrations_path: &str) -> Result<&'r Pool<Postgres>, DbConnectionError> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    Migrator::new(Path::new(migrations_path))
        .await?
        .run(&pool)
        .await?;

    Ok(CONNECTION.get_or_init(|| pool))
}