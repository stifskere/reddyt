use serde::{Deserialize, Serialize};
use thiserror::Error;
use chrono::{DateTime, Utc};

#[cfg(target_arch = "x86_64")]
use sqlx::{query, query_as, Error as SqlxError, PgPool, Type as SqlxType};
#[cfg(target_arch = "x86_64")]
use sqlx::prelude::FromRow;

#[cfg(target_arch = "x86_64")]
use crate::models::uploads::{UploadPlatform, UploadError};
use crate::models::uploads::Upload;

/// Represents errors when interacting with the `runs` table.
#[derive(Error, Debug)]
#[cfg(target_arch = "x86_64")]
pub enum RunError {
    /// Error occurring during a database query.
    #[error("Error querying the database, {0:#}")]
    QueryError(#[from] SqlxError),

    #[error("Error while obtaining uploads, {0:#}")]
    UploadError(#[from] UploadError),

    #[error(
        "{}{}{}",
        "Attempted to change an immutable run state. ",
        "Done and Error are immutable states and cannot be changed. ",
        "Attempting to do so will cause an error."
    )]
    FrozenState
}


/// Convenience result type for `Run` operations.
#[cfg(target_arch = "x86_64")]
type RunResult<T> = Result<T, RunError>;


/// Current processing state of a run.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(target_arch = "x86_64", derive(SqlxType))]
#[cfg_attr(target_arch = "x86_64", sqlx(type_name = "run_state", rename_all = "SCREAMING_SNAKE_CASE"))]
#[non_exhaustive]
pub enum RunState {
    /// The run encountered an error.
    Error,

    /// The creation state.
    Idling,

    /// The system is generating the question content.
    GeneratingQuestion,

    /// The system is generating the answer content.
    GeneratingAnswer,

    /// Voice rendering is in progress.
    RenderingVoice,

    /// Subtitles rendering is in progress.
    RenderingSubtitles,

    /// Background assets are being downloaded.
    DownloadingBackground,

    /// Video composition is in progress.
    ComposingVideo,

    /// Video upload is in progress.
    Uploading,

    /// Run has finished successfully.
    Done,
}


#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
#[cfg_attr(target_arch = "x86_64", derive(FromRow))]
#[cfg_attr(target_arch = "x86_64", sqlx(rename_all = "snake_case"))]
pub struct Run {
    /// Primary key of the run.
    id: i32,

    /// ID of the profile this run belongs to.
    profile_id: i32,

    /// Timestamp when the run started.
    run_date: DateTime<Utc>,

    /// Current processing state of the run.
    current_state: RunState,

    /// Optional error message if the run failed.
    error: Option<String>,
}


#[cfg(target_arch = "x86_64")]
impl RunState {
    /// Mutate the instance to the next state.
    ///
    /// # States
    /// **NOTE: Calling this method on the
    /// error or done state will have no effect.**
    ///
    /// ```text
    /// IDLING => GENERATING_QUESTION
    /// GENERATING_QUESTION => GENERATING_ANSWER
    /// GENERATING_ANSWER => RENDERING_VOICE
    /// RENDERING_VOICE => RENDERING_SUBTITLES
    /// RENDERING_SUBTITLES => DOWNLOADING_BACKGROUND
    /// DOWNLOADING_BACKGROUND => COMPOSING_VIDEO
    /// COMPOSING_VIDEO => UPLOADING
    /// UPLOADING => DONE
    /// ```
    ///
    /// In the case one of the steps errors this
    /// should be set to error.
    ///
    /// # Returns
    /// A shared mutable reference to
    /// the mutated instance.
    fn next_state(&mut self) -> &mut Self {
        *self = match &self {
            // Errors don't change.
            Self::Error => RunState::Error,

            // Sequential states.
            Self::Idling => Self::GeneratingQuestion,
            Self::GeneratingQuestion => Self::GeneratingAnswer,
            Self::GeneratingAnswer => Self::RenderingVoice,
            Self::RenderingVoice => Self::RenderingSubtitles,
            Self::RenderingSubtitles => Self::DownloadingBackground,
            Self::DownloadingBackground => Self::ComposingVideo,
            Self::ComposingVideo => Self::Uploading,

            // Finished state.
            | Self::Uploading
            | Self::Done => Self::Done,
        };

        self
    }


    /// Sets the current instance to error state.
    ///
    /// # Returns
    /// A shared mutable reference to
    /// the mutated instance.
    fn set_error(&mut self) -> &mut Self {
        *self = Self::Error;
        self
    }
}


#[cfg(target_arch = "x86_64")]
impl Run {
    /// Creates a new run for a profile.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    /// - `profile_id`: ID of the profile for which the run is created.
    ///
    /// # Returns
    /// - `Ok(Run)` if successfully inserted.
    /// - `Err(RunError)` if the query fails.
    #[must_use]
    pub(super) async fn create(connection: &PgPool, profile_id: i32) -> RunResult<Self> {
        let result = query_as(
            r"
                INSERT INTO runs (profile_id)
                VALUES ($1)
                RETURNING *
            "
        )
            .bind(profile_id)
            .fetch_one(connection)
            .await?;

        Ok(result)
    }


    /// Fetches a run by its ID.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    /// - `id`: ID of the run to fetch.
    ///
    /// # Returns
    /// - `Ok(Some(Run))` if found.
    /// - `Ok(None)` if no run exists with the given ID.
    /// - `Err(RunError)` if the query fails.
    #[must_use]
    pub async fn get(connection: &PgPool, id: i32) -> RunResult<Option<Self>> {
        let result = query_as(
            r"
                SELECT * FROM runs
                WHERE id = $1
            "
        )
            .bind(id)
            .fetch_optional(connection)
            .await?;

        Ok(result)
    }


    /// Fetch all runs associated with a specific profile.
    ///
    /// # Parameters
    /// - `connection`: Database connection.
    /// - `profile_id`: ID of the profile whose runs should be fetched.
    ///
    /// # Returns
    /// All runs related to the specified profile.
    #[must_use]
    pub async fn get_all_for_profile(
        connection: &PgPool,
        profile_id: i32
    ) -> RunResult<Vec<Self>> {
        let result = query_as(
            r#"
                SELECT *
                FROM runs
                WHERE profile_id = $1
                ORDER BY run_date DESC
            "#
        )
            .bind(profile_id)
            .fetch_all(connection)
            .await?;

        Ok(result)
    }


    /// Fetches all uploads associated with this run.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    ///
    /// # Returns
    /// - `Ok(Vec<Upload>)` containing all uploads for this run.
    /// - `Err(RunError)` if the query fails.
    #[must_use]
    pub async fn fetch_uploads(&self, connection: &PgPool) -> RunResult<Vec<Upload>> {
        Ok(
            Upload::get_all_for_run(connection, self.id).await?
        )
    }


    /// Create a new upload entry associated with this run.
    ///
    /// # Parameters
    /// - `connection`: Database connection.
    /// - `platform`: Platform where the content was uploaded.
    /// - `url`: Final URL of the uploaded content.
    ///
    /// # Returns
    /// The created upload entry.
    pub async fn create_upload(
        &self,
        connection: &PgPool,
        platform: UploadPlatform,
        url: String
    ) -> RunResult<Upload> {
        let upload = Upload::create(
            connection,
            self.id,
            platform,
            url,
        )
            .await?;

        Ok(upload)
    }


    /// Advances to the next possible run state.
    ///
    /// This will return an error if called on
    /// DONE or ERROR state, as these are immutable
    /// or frozen states.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    ///
    /// # Returns
    /// - `Ok(&mut Self)` if the update succeeded.
    /// - `Err(RunError)` if the query fails or a rule is broken.
    pub async fn update_state(
        &mut self,
        connection: &PgPool,
    ) -> RunResult<&mut Self> {
        if matches!(self.current_state, RunState::Error) {
            return Err(RunError::FrozenState);
        }

        // this will already update the instance state.
        let next_state = self.current_state.next_state();

        query(
            r"
                UPDATE runs
                SET current_state = $2
                WHERE id = $1
            "
        )
        .bind(self.id)
        .bind(*next_state)
        .execute(connection)
        .await?;

        Ok(self)
    }


    /// Sets this run's state to error.
    ///
    /// After this is called the run state
    /// will be considered immutable (frozen).
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    /// - `message`: Error message to be set.
    ///
    /// # Returns
    /// - `Ok(&mut Self)` if the update succeeded.
    /// - `Err(RunError)` if the query fails.
    pub async fn set_error(
        &mut self,
        connection: &PgPool,
        message: &str
    ) -> RunResult<&mut Self> {
        if matches!(self.current_state, RunState::Error) {
            return Err(RunError::FrozenState);
        }

        query(
            r"
                UPDATE runs
                SET error = $2
                WHERE id = $1
            "
        )
            .bind(self.id)
            .bind(message)
            .execute(connection)
            .await?;

        self.error = Some(message.to_string());
        Ok(self)
    }
}


impl Run {
    /// Returns the run ID.
    #[must_use]
    #[inline]
    pub fn id(&self) -> i32 {
        self.id
    }


    /// Returns the profile ID this run belongs to.
    #[must_use]
    #[inline]
    pub fn profile_id(&self) -> i32 {
        self.profile_id
    }


    /// Returns the timestamp when the run started.
    #[must_use]
    #[inline]
    pub fn run_date(&self) -> DateTime<Utc> {
        self.run_date
    }


    /// Returns the current processing state of the run.
    #[must_use]
    #[inline]
    pub fn current_state(&self) -> RunState {
        self.current_state
    }


    /// Returns the error message if the run failed.
    #[must_use]
    #[inline]
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
}
