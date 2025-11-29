use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};

#[derive(Serialize, Deserialize, Type, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[sqlx(type_name = "upload_platform_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UploadPlatformType {
	Local,
	Youtube
}

/// Model representation for upload platforms database schema.
#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, PartialOrd, Clone)]
pub struct UploadPlatform {
	id: i32,
	profile_id: i32,
	platform: UploadPlatformType,
	oauth_refresh: Vec<u8>,
	oauth_token: Vec<u8>
}
