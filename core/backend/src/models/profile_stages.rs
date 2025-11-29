use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


/// Model representation for profile stage database schema.
#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, PartialOrd, Clone)]
pub struct ProfileStage {
	/// The primary key for this model.
	id: i32,

	/// The profile this video stage belongs to.
	profile_id: i32,

	/// The name of this stage to be referenced by other stages.
	name: String,

	/// The FIFO stage connection order, if -1 is the first stage,
	/// otherwise the last stage ID. If null the node is disconnected.
	last_stage: Option<i32>
}
