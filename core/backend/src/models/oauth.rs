use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(target_arch = "x86_64")]
use sqlx::{query, query_as, Error as SqlxError, PgPool, Type as SqlxType};
#[cfg(target_arch = "x86_64")]
use sqlx::prelude::FromRow;


/// Represents all possible errors when interacting with the `profile_oauth` table.
#[derive(Error, Debug)]
pub enum ProfileOAuthError {
    /// Error that occurs when a database query fails.
    ///
    /// Wraps any `sqlx::Error` returned by SQLx operations.
    #[error("Error querying the database, {0:#}")]
    QueryError(#[from] SqlxError),
}


/// Convenience result type used throughout the `ProfileOAuth` module.
///
/// Returns either a successful value `T` or a `ProfileOAuthError`.
#[allow(unused)]
type ProfileOAuthResult<T> = Result<T, ProfileOAuthError>;


/// Supported OAuth providers for a profile.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(target_arch = "x86_64", derive(SqlxType))]
#[cfg_attr(target_arch = "x86_64", sqlx(type_name = "oauth_type", rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum OAuthType {
    /// OAuth provider for Youtube.
    Youtube
}


/// Represents a single OAuth token set for a profile.
///
/// Maps directly to the `profile_oauth` database table.
#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
#[cfg_attr(target_arch = "x86_64", derive(FromRow))]
#[cfg_attr(target_arch = "x86_64", sqlx(rename_all = "snake_case"))]
pub struct ProfileOAuth {
    /// Primary key of the token set.
    id: i32,

    /// ID of the profile that owns this token set.
    profile_id: i32,

    /// OAuth provider type (e.g., Youtube).
    oauth_type: OAuthType,

    /// Optional refresh token issued by the provider.
    /// Can be `None` if not provided or expired.
    refresh_token: Option<String>,

    /// Optional authentication token issued by the provider.
    /// Can be `None` if not provided or expired.
    auth_token: Option<String>
}


#[cfg(target_arch = "x86_64")]
impl ProfileOAuth {
    /// Creates a new `ProfileOAuth` token set for the specified profile.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    /// - `profile_id`: ID of the profile to attach the OAuth token to.
    /// - `oauth_type`: The OAuth provider type.
    /// - `refresh_token`: Optional refresh token.
    /// - `auth_token`: Optional authentication token.
    ///
    /// # Returns
    /// - `Ok(ProfileOAuth)` if the row is successfully inserted.
    /// - `Err(ProfileOAuthError)` if the query fails.
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


    /// Fetches a `ProfileOAuth` token set by `profile_id` and `oauth_type`.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    /// - `profile_id`: ID of the profile the token set is from.
    /// - `oauth_type`: The OAuth provider type.
    ///
    /// # Returns
    /// - `Ok(Some(ProfileOAuth))` if a matching row exists.
    /// - `Ok(None)` if no matching row is found.
    /// - `Err(ProfileOAuthError)` if the query fails.
    #[must_use]
    #[allow(unused)]
    pub(super) async fn get(
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


    /// Fetches all `ProfileOAuth` token sets for a given `profile_id`.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    /// - `profile_id`: ID of the token sets owner..
    ///
    /// # Returns
    /// - `Ok(Vec<ProfileOAuth>)` if successful.
    /// - `Err(ProfileOAuthError)` if the query fails.
    #[must_use]
    pub(super) async fn get_all(connection: &PgPool, profile_id: i32) -> ProfileOAuthResult<Vec<Self>> {
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


    /// Deletes this `ProfileOAuth` token set from the database.
    ///
    /// # Parameters
    /// - `connection`: Reference to the database pool.
    ///
    /// # Returns
    /// - `Ok(Some(()))` if deletion succeeded.
    /// - `Ok(None)` if no rows matched.
    /// - `Err(ProfileOAuthError)` if the query fails.
    pub async fn delete(self, connection: &PgPool) -> ProfileOAuthResult<Option<()>> {
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


    /// Updates the `refresh_token` field in the database and this instance.
    ///
    /// # Parameters
    /// - `new_token`: New refresh token value or `None` to clear.
    ///
    /// # Returns
    /// - `Ok(&mut Self)` if successful.
    /// - `Err(ProfileOAuthError)` if the query fails.
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


    /// Updates the `auth_token` field in the database and this instance.
    ///
    /// # Parameters
    /// - `new_token`: New authentication token or `None` to clear.
    ///
    /// # Returns
    /// - `Ok(&mut Self)` if successful.
    /// - `Err(ProfileOAuthError)` if the query fails.
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
    /// Returns the token set's ID.
    #[must_use]
    #[inline]
    pub fn id(&self) -> i32 {
        self.id
    }


    /// Returns the profile ID owning this token set.
    #[must_use]
    #[inline]
    pub fn profile_id(&self) -> i32 {
    self.profile_id
    }


    /// Returns the OAuth provider type.
    #[must_use]
    #[inline]
    pub fn oauth_type(&self) -> OAuthType {
        self.oauth_type
    }


    /// Returns the refresh token, if any.
    #[must_use]
    #[inline]
    pub fn refresh_token(&self) -> &Option<String> {
        &self.refresh_token
    }


    /// Returns the authentication token, if any.
    #[must_use]
    #[inline]
    pub fn auth_token(&self) -> &Option<String> {
        &self.auth_token
    }
}
