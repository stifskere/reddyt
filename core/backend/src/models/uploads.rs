use serde::{Deserialize, Serialize};
use thiserror::Error;
use chrono::{DateTime, Utc};

#[cfg(target_arch = "x86_64")]
use sqlx::{query, query_as, Error as SqlxError, PgPool, Type as SqlxType};
#[cfg(target_arch = "x86_64")]
use sqlx::prelude::FromRow;


/// Represents all possible errors when interacting with the `uploads` table.
#[derive(Error, Debug)]
pub enum UploadError {
    /// Error that occurs when a database query fails.
    ///
    /// Wraps any `sqlx::Error` returned by SQLx operations.
    #[error("Error querying the database, {0:#}")]
    QueryError(#[from] SqlxError),
}


/// Convenience result type used throughout the `Upload` module.
type UploadResult<T> = Result<T, UploadError>;


/// Supported upload platforms.
///
/// WARNING: Modifying the configuration or target
/// URL can lead to incorrect provider results.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(target_arch = "x86_64", derive(SqlxType))]
#[cfg_attr(target_arch = "x86_64", sqlx(type_name = "upload_platform", rename_all = "SCREAMING_SNAKE_CASE"))]
#[non_exhaustive]
pub enum UploadPlatform {
    /// Video uploaded via the configured local provider.
    LocalProvider,

    /// Video uploaded to YouTube.
    Youtube,
}


/// Represents an uploaded video for a specific run.
///
/// Maps directly to the `uploads` table in the database.
///
/// WARNING: Modifying the configuration or target
/// URL can lead to incorrect provider results.
#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
#[cfg_attr(target_arch = "x86_64", derive(FromRow))]
#[cfg_attr(target_arch = "x86_64", sqlx(rename_all = "snake_case"))]
pub struct Upload {
    /// Primary key of the upload.
    id: i32,

    /// ID of the run this upload belongs to.
    run_id: i32,

    /// Platform to which the video was uploaded.
    platform: UploadPlatform,

    /// URL of the uploaded video.
    url: String,

    /// Timestamp when the upload occurred.
    uploaded_at: DateTime<Utc>,
}


#[cfg(target_arch = "x86_64")]
impl Upload {
    /// Creates a new upload entry in the database.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    /// - `run_id`: ID of the run this upload belongs to.
    /// - `platform`: Platform the video was uploaded to.
    /// - `url`: URL of the uploaded video.
    ///
    /// # Returns
    /// - `Ok(Upload)` if the row was successfully inserted.
    /// - `Err(UploadError)` if the query fails.
    #[must_use]
    pub(super) async fn create(
        connection: &PgPool,
        run_id: i32,
        platform: UploadPlatform,
        url: String
    ) -> UploadResult<Self> {
        let result = query_as(
            r"
                INSERT INTO uploads (run_id, platform, url)
                VALUES ($1, $2, $3)
                RETURNING *
            "
        )
        .bind(run_id)
        .bind(platform)
        .bind(url)
        .fetch_one(connection)
        .await?;

        Ok(result)
    }


    /// Fetches all uploads for a given run.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    /// - `run_id`: ID of the run whose uploads to fetch.
    ///
    /// # Returns
    /// - `Ok(Vec<Upload>)` if successful.
    /// - `Err(UploadError)` if the query fails.
    #[must_use]
    pub(crate) async fn get_all_for_run(
        connection: &PgPool,
        run_id: i32
    ) -> UploadResult<Vec<Self>> {
        let result = query_as(
            r"
                SELECT * FROM uploads
                WHERE run_id = $1
            "
        )
        .bind(run_id)
        .fetch_all(connection)
        .await?;

        Ok(result)
    }


    /// Deletes this upload entry from the database.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    ///
    /// # Returns
    /// - `Ok(Some(()))` if deletion succeeded.
    /// - `Ok(None)` if no rows were affected.
    /// - `Err(UploadError)` if the query fails.
    pub async fn delete(
        self,
        connection: &PgPool
    ) -> UploadResult<Option<()>> {
        let result = query(
            r"
                DELETE FROM uploads
                WHERE id = $1
            "
        )
        .bind(self.id)
        .execute(connection)
        .await?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        Ok(Some(()))
    }
}


impl Upload {
    /// Returns the upload's ID.
    #[must_use]
    #[inline]
    pub fn id(&self) -> i32 {
        self.id
    }


    /// Returns the run ID associated with this upload.
    #[must_use]
    #[inline]
    pub fn run_id(&self) -> i32 {
        self.run_id
    }


    /// Returns the upload platform.
    #[must_use]
    #[inline]
    pub fn platform(&self) -> UploadPlatform {
        self.platform
    }


    /// Returns the URL of the uploaded video.
    #[must_use]
    #[inline]
    pub fn url(&self) -> &str {
        &self.url
    }


    /// Returns the timestamp when the upload occurred.
    #[must_use]
    #[inline]
    pub fn uploaded_at(&self) -> DateTime<Utc> {
        self.uploaded_at
    }
}
