use sqlx::migrate::{MigrateError, Migrator};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error as SqlxError, Pool, Postgres};
use std::{path::Path, sync::OnceLock};
use thiserror::Error;

#[macro_export]
macro_rules! db {
    () => {
        $crate::helpers::database::connection::get_db_connection().await?
    };
}

static CONNECTION: OnceLock<Pool<Postgres>> = OnceLock::new();

#[derive(Error, Debug)]
pub enum DbConnectionError {
    #[error("{0:#}")]
    ConnectionError(#[from] SqlxError),

    #[error("{0:#}")]
    MigrateError(#[from] MigrateError),
}

pub async fn get_db_connection<'r>() -> Result<&'r Pool<Postgres>, DbConnectionError> {
    if let Some(connection) = CONNECTION.get() {
        return Ok(connection);
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(env!("DATABASE_URL"))
        .await?;

    Migrator::new(Path::new(&option_env!("DATABASE_MIGRATIONS").unwrap_or("./migrations")))
        .await?
        .run(&pool)
        .await?;

    Ok(CONNECTION.get_or_init(|| pool))
}
