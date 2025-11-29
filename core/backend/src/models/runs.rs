use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


/// Model representation for runs database schema.
#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, PartialOrd, Clone)]
pub struct Run {
	/// The primary key for this model.
	id: i32,

	/// The profile this run belongs to.
	profile_id: i32,

	/// If there was an error while running it's `Display`
	/// is going to be stored here.
	error: Option<String>,

	/// What layer is being processed at this moment, the
	/// format is `stage.layer`.
	processing: Vec<String>,

	/// When did this start running, this is used
	/// by the scheduler to know if it should start a new run.
	started_at: DateTime<Utc>,

	/// When did this end running, this is used
	/// by the UI to display the running state.
	finished_at: DateTime<Utc>
}
