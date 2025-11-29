use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


/// Model representation for uploads database schema.
#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, PartialOrd, Clone)]
pub struct Uploads {
	id: i32,
	upload_platform_id: i32,
	run_id: i32,
	generated_url: i32,
	uploaded_at: DateTime<Utc>
}
