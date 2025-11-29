use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


/// Model representation for accounts database schema.
#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, PartialOrd, Clone)]
pub struct Account {
	id: i32,
	email: String,
	password: Vec<u8>
}
