use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


/// Model representation for accounts database schema.
#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, PartialOrd, Clone)]
pub struct Account {
	/// The primary key for this model.
	id: i32,
	
	/// The account email.
	email: String,

	/// The account password.
	password: Vec<u8>
}

impl Account {
	/// The primary key for this model.
    pub fn id(&self) -> i32 {
        self.id
    }

	/// The account email.
    pub fn email(&self) -> &str {
        &self.email
    }

	/// The account password.
    pub fn password(&self) -> &[u8] {
        &self.password
    }
}
