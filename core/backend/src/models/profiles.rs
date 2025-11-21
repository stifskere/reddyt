use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(target_arch = "x86_64")]
use sqlx::{query_as, Error as SqlxError, PgPool};
#[cfg(target_arch = "x86_64")]
use sqlx::prelude::FromRow;

#[cfg(target_arch = "x86_64")]
use crate::models::profile_overrides::{ProfileOverride, ProfileOverridesError};
#[cfg(target_arch = "x86_64")]
use crate::models::oauth::{ProfileOAuth, ProfileOAuthError, OAuthType};
#[cfg(target_arch = "x86_64")]
use crate::models::uploads::{Upload, UploadError, UploadPlatform};
#[cfg(target_arch = "x86_64")]
use crate::models::runs::{Run, RunError};

/// Errors for interacting with the `profiles` table and its relations.
#[derive(Error, Debug)]
#[cfg(target_arch = "x86_64")]
pub enum ProfileError {
    /// Database query failed.
    #[error("Error querying the database, {0:#}")]
    QueryError(#[from] SqlxError),

    #[error("Error creating or fetching runs, {0:#}")]
    RunError(#[from] RunError),

    #[error("Error creating or fetching OAuth connections, {0:#}")]
    OAuthError(#[from] ProfileOAuthError),

    #[error("Error creating or fetching overrides, {0:#}")]
    OverrideError(#[from] ProfileOverridesError),

    #[error("Error when handling uploads, {0:#}")]
    UploadError(#[from] UploadError),
}


type ProfileResult<T> = Result<T, ProfileError>;


/// Represents a user's profile.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(target_arch = "x86_64", derive(FromRow))]
#[cfg_attr(target_arch = "x86_64", sqlx(rename_all = "snake_case"))]
pub struct Profile {
    id: i32,

    /// A human-readable identifier with no operational effect.
    name: String,

    /// Cron expression used to populate the next run time.
    upload_schedule: String,

    /// If true, scheduled runs are suppressed.
    paused: bool,

    /// Prompt used to generate a “reddyt-style question”.
    question_prompt: String,

    /// Prompt used for answering the above question.
    answer_prompt: String,

    /// Glob filter for selecting backgrounds from the storage provider.
    background_glob: String,

    /// Voice identifier used by the TTS provider.
    voice_name: String,

    /// Font name (Google Fonts identifier) for caption generation.
    font_name: String,
}


#[cfg(target_arch = "x86_64")]
impl Profile {
    /// Creates a new profile row.
    ///
    /// - `connection`: Reference to the database pool.
    ///
    /// # Returns
    /// - `Ok(Profile)` if inserted.
    /// - `Err(ProfileError)` if query fails.
    #[must_use]
    pub(super) async fn create(connection: &PgPool) -> ProfileResult<Self> {
        let result = query_as(
            r"
                INSERT INTO profiles DEFAULT VALUES
                RETURNING *
            "
        )
        .fetch_one(connection)
        .await?;

        Ok(result)
    }


    /// Fetches a profile by ID.
    ///
    /// - `connection`: Reference to the database pool.
    /// - `profile_id`: The profile ID to fetch.
    #[must_use]
    pub async fn get(connection: &PgPool, profile_id: i32) -> ProfileResult<Option<Self>> {
        let result = query_as(
            r"
                SELECT * FROM profiles
                WHERE id = $1
            "
        )
        .bind(profile_id)
        .fetch_optional(connection)
        .await?;

        Ok(result)
    }


    /// Creates a new `Run` associated with this profile.
    ///
    /// Wraps `runs::Run::create`.
    #[must_use]
    pub async fn create_run(&self, connection: &PgPool) -> ProfileResult<Run> {
        let run = Run::create(connection, self.id).await?;
        Ok(run)
    }


    /// Fetches all runs for this profile.
    #[must_use]
    pub async fn fetch_runs(&self, connection: &PgPool) -> ProfileResult<Vec<Run>> {
        let runs = Run::get_all_for_profile(connection, self.id).await?;
        Ok(runs)
    }


    /// Adds a new OAuth connection for this profile.
    ///
    /// Wraps `ProfileOAuth::create`.
    #[must_use]
    pub async fn add_oauth_connection(
        &self,
        connection: &PgPool,
        provider: OAuthType,
        refresh_token: Option<String>,
        auth_token: Option<String>
    ) -> ProfileResult<ProfileOAuth> {
        let oauth = ProfileOAuth::create(
            connection,
            self.id,
            provider,
            refresh_token,
            auth_token
        ).await?;

        Ok(oauth)
    }


    /// Fetches all OAuth connections for this profile.
    #[must_use]
    pub async fn fetch_oauth_connections(
        &self,
        connection: &PgPool
    ) -> ProfileResult<Vec<ProfileOAuth>> {
        let result = ProfileOAuth::get_all_for_profile(connection, self.id).await?;
        Ok(result)
    }


    /// Creates a new profile override.
    ///
    /// Wraps `ProfileOverride::create`.
    #[must_use]
    pub async fn create_override(
        &self,
        connection: &PgPool,
        runs_at: chrono::DateTime<chrono::Utc>
    ) -> ProfileResult<ProfileOverride> {
        let ov = ProfileOverride::create(connection, self.id, runs_at).await?;
        Ok(ov)
    }


    /// Fetches all overrides for this profile.
    #[must_use]
    pub async fn fetch_overrides(
        &self,
        connection: &PgPool
    ) -> ProfileResult<Vec<ProfileOverride>> {
        let result = ProfileOverride::get_all_for_profile(connection, self.id).await?;
        Ok(result)
    }
}


impl Profile {
    /// Returns profile ID.
    #[inline]
    #[must_use]
    pub fn id(&self) -> i32 {
        self.id
    }


    /// Returns profile name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }


    /// Returns upload schedule cron string.
    #[inline]
    #[must_use]
    pub fn upload_schedule(&self) -> &str {
        &self.upload_schedule
    }


    /// Returns true if the profile is paused.
    #[inline]
    #[must_use]
    pub fn paused(&self) -> bool {
        self.paused
    }


    /// Returns the question prompt.
    #[inline]
    #[must_use]
    pub fn question_prompt(&self) -> &str {
        &self.question_prompt
    }


    /// Returns the answer prompt.
    #[inline]
    #[must_use]
    pub fn answer_prompt(&self) -> &str {
        &self.answer_prompt
    }


    /// Returns the background glob filter.
    #[inline]
    #[must_use]
    pub fn background_glob(&self) -> &str {
        &self.background_glob
    }


    /// Returns the voice name for TTS.
    #[inline]
    #[must_use]
    pub fn voice_name(&self) -> &str {
        &self.voice_name
    }


    /// Returns the font name for captions.
    #[inline]
    #[must_use]
    pub fn font_name(&self) -> &str {
        &self.font_name
    }
}
