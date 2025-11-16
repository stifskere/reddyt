use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(target_arch = "x86_64")]
use sqlx::{query, query_as, Error as SqlxError, PgPool, Type as SqlxType};
#[cfg(target_arch = "x86_64")]
use sqlx::prelude::FromRow;


/// Enum declaration for errors happening within
/// the profile oauth database transactions.
#[derive(Error, Debug)]
pub enum ProfileOAuthModelError {
    #[error("Error querying the database, {0:#}")]
    QueryError(#[from] SqlxError),
}

/// Shorter error type definition for errors returning [`ProfileOAuthModelError`]
/// as error variant.
#[allow(unused)]
type ProfileOAuthResult<T> = Result<T, ProfileOAuthModelError>;

/// Enum declaration identifying all possible types of
/// OAuth connections for a profile.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(target_arch = "x86_64", derive(SqlxType))]
#[cfg_attr(target_arch = "x86_64", sqlx(type_name = "oauth_type", rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum OAuthType {
    Youtube
}

/// `profile_oauth` database relation struct.
///
/// This is a token set for OAuth connections within profiles.
#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
#[cfg_attr(target_arch = "x86_64", derive(FromRow))]
#[cfg_attr(target_arch = "x86_64", sqlx(rename_all = "snake_case"))]
pub struct ProfileOAuth {
    id: i32,
    profile_id: i32,
    oauth_type: OAuthType,
    refresh_token: Option<String>,
    auth_token: Option<String>
}

#[cfg(target_arch = "x86_64")]
impl ProfileOAuth {
    /// `profile_oauth` relation creation for `profile`, this is encapsualted at
    /// module level and should be only exposed within a wrapper function
    /// in the profile relation itself.
    ///
    /// Returns Ok(Self) if the relation could be created
    /// or Err(..) if there was a query error.
    #[must_use]
    pub(super) async fn create(
        connection: &PgPool,
        profile_id: i32,
        oauth_type: OAuthType,
        refresh_token: Option<String>,
        auth_token: Option<String>
    ) -> ProfileOAuthResult<Self> {
        let result = query_as(
            r"
                INSERT INTO profile_oauth (
                    profile_id,
                    oauth_type,
                    refresh_token,
                    auth_token
                )
                VALUES (
                    $1,
                    $2,
                    $3,
                    $4
                )
            "
        )
            .bind(profile_id)
            .bind(oauth_type)
            .bind(refresh_token)
            .bind(auth_token)
            .fetch_one(connection)
            .await?;

        Ok(result)
    }

    /// Fetch a `profile_oauth` relation from the
    /// database, this is encapsulated at module
    /// for lazy level for loading within a
    /// `profile` relation.
    ///
    /// Returns Ok(Self) if success,
    /// Ok(None) if no matching model exists,
    /// and Err(..) if there was a query error.
    #[must_use]
    #[allow(unused)]
    pub(crate) async fn get(
        connection: &PgPool,
        profile_id: i32,
        oauth_type: OAuthType
    ) -> ProfileOAuthResult<Option<Self>> {
        let result = query_as(
            r"
                SELECT * FROM profile_oauth
                WHERE
                    profile_id = $1
                AND
                    oauth_type = $2
            "
        )
            .bind(profile_id)
            .bind(oauth_type)
            .fetch_optional(connection)
            .await?;

        Ok(result)
    }

    /// Obtain all the `profile_oauth` relations
    /// matching a `profile_id`.
    ///
    /// Returns Ok(Vec<Self>) if success
    /// or Err(..) if a query error occurred.
    #[must_use]
    pub(crate) async fn get_all(connection: &PgPool, profile_id: i32) -> ProfileOAuthResult<Vec<Self>> {
        let result = query_as(
            r"
                SELECT * FROM profile_oauth
                WHERE
                    profile_id = $1
            "
        )
            .bind(profile_id)
            .fetch_all(connection)
            .await?;

        Ok(result)
    }

    /// Deletes a `profile_oauth` relation from the
    /// database and consumes the instance.
    ///
    /// To ensure the model actually exists before deleting
    /// it, it's deleted from an already selected model.
    ///
    /// This introduces overhead on selecting a model,
    /// but since postgres and rust are fast enough
    /// we can allow us that.
    ///
    /// THIS IS NOT ATOMIC.
    ///
    /// Returns Ok(Some(())) if deletion was successful,
    /// Ok(None) if no rows were matching
    /// and Err(..) if there was an error while querying.
    pub(crate) async fn delete(self, connection: &PgPool) -> ProfileOAuthResult<Option<()>> {
        let result = query(
            r"
                DELETE FROM profile_oauth
                WHERE
                    id = $1
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

    /// Update the `refresh_token` column from the
    /// database, as well as this instance field.
    ///
    /// There is no remove method, this will
    /// be set to null if `None` is passed.
    ///
    /// Returns Ok(&mut Self) if success
    /// otherwise Err(..)
    pub async fn update_refresh_token(
        &mut self,
        connection: &PgPool,
        new_token: Option<String>
    ) -> ProfileOAuthResult<&mut Self> {
        query(
            r"
                UPDATE profile_oauth
                SET
                    refresh_token = $2
                WHERE
                    id = $1
            "
        )
            .bind(self.id)
            .bind(&new_token)
            .execute(connection)
            .await?;

        self.refresh_token = new_token;

        Ok(self)
    }

    /// Update the `auth_token` column from the
    /// database, as well as this instance field.
    ///
    /// There is no remove method, this will
    /// be set to null if `None` is passed.
    ///
    /// Returns Ok(&mut Self) if success
    /// otherwise Err(..)
    pub async fn update_auth_token(
        &mut self,
        connection: &PgPool,
        new_token: Option<String>
    ) -> ProfileOAuthResult<&mut Self> {
        query(
            r"
                UPDATE profile_oauth
                SET
                    auth_token = $2
                WHERE
                    id = $1
            "
        )
            .bind(self.id)
            .bind(&new_token)
            .execute(connection)
            .await?;

        self.auth_token = new_token;

        Ok(self)
    }

}

impl ProfileOAuth {
    /// The profile OAuth token set relation id.
    #[must_use]
    #[inline]
    pub fn id(&self) -> i32 {
        self.id
    }

    /// The owner profile relation id.
    #[must_use]
    #[inline]
    pub fn profile_id(&self) -> i32 {
        self.profile_id
    }

    /// The OAuth platform for this token set.
    #[must_use]
    #[inline]
    pub fn oauth_type(&self) -> OAuthType {
        self.oauth_type
    }

    /// The token set refresh token.
    ///
    /// This is optional depending on the platform.
    #[must_use]
    #[inline]
    pub fn refresh_token(&self) -> &Option<String> {
        &self.refresh_token
    }

    /// The token set auth token.
    ///
    /// This is optional depending on the platform.
    #[must_use]
    #[inline]
    pub fn auth_token(&self) -> &Option<String> {
        &self.auth_token
    }
}
