use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


/// Model representation for runs database schema.
#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, PartialOrd, Clone)]
pub struct Run {
	id: i32,
	profile_id: i32,
	error: Option<String>,
	processing: Vec<String>,
	started_at: DateTime<Utc>,
	finished_at: DateTime<Utc>
}
