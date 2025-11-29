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

impl ProfileStageLayer {
	/// The primary key for this model.
    pub fn id(&self) -> i32 {
        self.id
    }

	/// Which stage does this layer belong to.
    pub fn video_stage_id(&self) -> i32 {
        self.video_stage_id
    }

	/// In which order ASC, largest covering smallest.
    pub fn order(&self) -> i32 {
        self.order
    }

	/// The raw layer data to be processed by bincode.
    pub fn layer_data(&self) -> &[u8] {
        &self.layer_data
    }
}
