use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


/// Model representation for profile stage database schema.
#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, PartialOrd, Clone)]
pub struct ProfileStage {
	id: i32,
	profile_id: i32,
	name: String,
	last_stage: Option<i32>
}
