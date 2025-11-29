use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


/// Model representation for profile stage layer database schema.
#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, PartialOrd, Clone)]
pub struct ProfileStageLayer {
	/// The primary key for this model.
	id: i32,

	/// Which stage does this layer belong to.
	video_stage_id: i32,

	/// In which order ASC, largest covering smallest.
	order: i32,

	/// The raw layer data to be processed by bincode.
	layer_data: Vec<u8>
}
