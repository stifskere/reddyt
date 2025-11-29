use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


/// Model representation for profile stage layer database schema.
#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, PartialOrd, Clone)]
pub struct ProfileStageLayer {
	id: i32,
	video_stage_id: i32,
	order: i32,
	layer_data: Vec<u8>
}
