use serde::{Deserialize, Serialize};
use thiserror::Error;
use chrono::{DateTime, Utc};

#[cfg(target_arch = "x86_64")]
use sqlx::{query, query_as, Error as SqlxError, PgPool};
#[cfg(target_arch = "x86_64")]
use sqlx::prelude::FromRow;


/// Represents all possible errors when interacting with the `profile_overrides` table.
#[derive(Error, Debug)]
pub enum ProfileOverridesError {
    /// Error that occurs when a database query fails.
    ///
    /// Wraps any `sqlx::Error` returned by SQLx operations.
    #[error("Error querying the database, {0:#}")]
    QueryError(#[from] SqlxError),
}


/// Convenience result type used throughout the `ProfileOverrides` module.
///
/// Returns either a successful value `T` or a `ProfileOverridesError`.
#[allow(unused)]
type ProfileOverridesResult<T> = Result<T, ProfileOverridesError>;


/// Represents a scheduling override for a profile.
///
/// Maps directly to the `profile_overrides` database table.
#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
#[cfg_attr(target_arch = "x86_64", derive(FromRow))]
#[cfg_attr(target_arch = "x86_64", sqlx(rename_all = "snake_case"))]
pub struct ProfileOverrides {
    /// Primary key of the override entry.
    id: i32,

    /// ID of the profile this override belongs to.
    profile_id: i32,

    /// Timestamp when the override is scheduled to run.
    runs_at: DateTime<Utc>,

    /// Whether the override has been claimed.
    claimed: bool
}

#[cfg(target_arch = "x86_64")]
impl ProfileOverrides {
    /// Creates a new `ProfileOverrides` entry in the database.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    /// - `profile_id`: ID of the profile to associate the override with.
    /// - `runs_at`: Scheduled timestamp for the override to run.
    ///
    /// # Returns
    /// - `Ok(ProfileOverrides)` if successfully inserted.
    /// - `Err(ProfileOverridesError)` if the query fails.
    #[must_use]
    pub(super) async fn create(
        connection: &PgPool,
        profile_id: i32,
        runs_at: DateTime<Utc>
    ) -> ProfileOverridesResult<Self> {
        let result = query_as(
            r"
                INSERT INTO profile_overrides (
                    profile_id,
                    runs_at,
                    claimed
                )
                VALUES (
                    $1,
                    $2,
                    false
                )
            "
        )
            .bind(profile_id)
            .bind(runs_at)
            .fetch_one(connection)
            .await?;

        Ok(result)
    }


    /// Fetches a `ProfileOverrides` entry by its ID.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    /// - `id`: ID of the override entry to fetch.
    ///
    /// # Returns
    /// - `Ok(Some(ProfileOverrides))` if a matching row exists.
    /// - `Ok(None)` if no matching row is found.
    /// - `Err(ProfileOverridesError)` if the query fails.
    #[must_use]
    pub(super) async fn get(
        connection: &PgPool,
        id: i32
    ) -> ProfileOverridesResult<Option<Self>> {
        let result = query_as(
            r"
                SELECT * FROM profile_overrides
                WHERE
                    id = $1
            "
        )
            .bind(id)
            .fetch_optional(connection)
            .await?;

        Ok(result)
    }


    /// Fetches all `ProfileOverrides` entries for a given profile.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    /// - `profile_id`: ID of the profile whose overrides to fetch.
    ///
    /// # Returns
    /// - `Ok(Vec<ProfileOverrides>)` if successful.
    /// - `Err(ProfileOverridesError)` if the query fails.
    #[must_use]
    pub(super) async fn get_all(
        connection: &PgPool,
        profile_id: i32
    ) -> ProfileOverridesResult<Vec<Self>> {
        let result = query_as(
            r"
                SELECT * FROM profile_overrides
                WHERE
                    profile_id = $1
            "
        )
            .bind(profile_id)
            .fetch_all(connection)
            .await?;

        Ok(result)
    }


    /// Deletes this `ProfileOverrides` entry from the database.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    ///
    /// # Returns
    /// - `Ok(Some(()))` if deletion succeeded.
    /// - `Ok(None)` if no rows were affected.
    /// - `Err(ProfileOverridesError)` if the query fails.
    pub async fn delete(
        self,
        connection: &PgPool
    ) -> ProfileOverridesResult<Option<()>> {
        let result = query(
            r"
                DELETE FROM profile_overrides
                WHERE
                    id = $1
            "
        )
            .bind(self.id)
            .execute(connection)
            .await?;

        if result.rows_affected() > 0 {
            return Ok(None);
        }

        Ok(Some(()))
    }


    /// Updates the scheduled timestamp (`runs_at`) for this `ProfileOverrides` entry
    /// in the database and updates the instance accordingly.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    /// - `new_date`: The new `DateTime<Utc>` value to set for the `runs_at` field.
    ///
    /// # Returns
    /// - `Ok(&mut Self)` if the update was successful.
    /// - `Err(ProfileOverridesError)` if the query fails.
    pub async fn update_run_date(
        &mut self,
        connection: &PgPool,
        new_date: DateTime<Utc>
    ) -> ProfileOverridesResult<&mut Self> {
        if self.runs_at == new_date {
            return Ok(self);
        }

        query(
            r"
                UPDATE profile_overrides
                SET
                    runs_at = $2
                WHERE
                    id = $1
            "
        )
            .bind(self.id)
            .bind(new_date)
            .execute(connection)
            .await?;

        self.runs_at = new_date;

        Ok(self)
    }


    /// Marks this `ProfileOverrides` entry as claimed in the database.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    ///
    /// # Returns
    /// - `Ok(&mut Self)` if the entry was successfully marked as claimed or was
    ///   already claimed.
    /// - `Err(ProfileOverridesError)` if the database query fails.
    pub async fn claim(
        &mut self,
        connection: &PgPool
    ) -> ProfileOverridesResult<&mut Self> {
        if self.claimed {
            return Ok(self);
        }

        query(
            r"
                UPDATE profile_overrides
                SET
                    claimed = true
                WHERE
                    id = $1
            "
        )
            .bind(self.id)
            .execute(connection)
            .await?;

        self.claimed = true;

        Ok(self)
    }
}

impl ProfileOverrides {
    /// Returns the ID of the override entry.
    pub fn id(&self) -> i32 {
        self.id
    }


    /// Returns the profile ID associated with this override.
    pub fn profile_id(&self) -> i32 {
        self.profile_id
    }


    /// Returns the scheduled timestamp for this override.
    pub fn runs_at(&self) -> DateTime<Utc> {
        self.runs_at
    }


    /// Returns whether this override has been claimed or already ran.
    pub fn claimed(&self) -> bool {
        self.claimed
    }
}
