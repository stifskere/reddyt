use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};

/// The target platforms to upload 
#[derive(Serialize, Deserialize, Type, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[sqlx(type_name = "upload_platform_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UploadPlatformType {
	/// The provider defined in the environment variables.
	Local,

	/// YouTube Short Video Upload.
	YoutubeShorts,
	/// YouTube Long Form Video Upload.
	YoutubeVideo
}

/// Model representation for upload platforms database schema.
#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, PartialOrd, Clone)]
pub struct UploadPlatform {
	/// The primary key for this model.
	id: i32,

	/// The profile this credential set belongs to.
	profile_id: i32,

	//// Which platform is this credential set from.
	platform: UploadPlatformType,

	/// The credential set OAuth refresh token.
	oauth_refresh: Vec<u8>,

	/// The credential set OAuth secret token.
	oauth_token: Vec<u8>
}

impl UploadPlatform {
	/// The primary key for this model.
    pub fn id(&self) -> i32 {
        self.id
    }

	/// The profile this credential set belongs to.
    pub fn profile_id(&self) -> i32 {
        self.profile_id
    }

	//// Which platform is this credential set from.
    pub fn platform(&self) -> UploadPlatformType {
        self.platform
    }

	/// The credential set OAuth refresh token.
    pub fn oauth_refresh(&self) -> &[u8] {
        &self.oauth_refresh
    }

	/// The credential set OAuth secret token.
    pub fn oauth_token(&self) -> &[u8] {
        &self.oauth_token
    }
}
