use scrypt::password_hash::rand_core::OsRng;
use scrypt::password_hash::{PasswordHasher, SaltString, Error as PasswordHashError};
use scrypt::Scrypt;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool, Error as SqlxError};
use sqlx::prelude::FromRow;
use thiserror::Error;


/// Represents solely server side errors, any client
/// error such as existing email, invalid password...
/// Should have their own wrapper.
#[derive(Debug, Error)]
pub enum AccountError {
	#[error("Error while querying the database, {0:#}")]
	DatabaseConnection(#[from] SqlxError),

	#[error("Error with password hashing operations, {0:#}")]
	PasswordHash(#[from] PasswordHashError)
}


/// Wraps account creation client errors,
/// this also contains the possibility of
/// a created account.
pub enum AccountCreationResult {
	/// The account was successfully created.
	Created(Account),

	/// The account already exists, checked against email.
	AlreadyExists
}


/// Database interface for account credentials.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum AccountCredentials {
	/// Basic Authentication / email + UNENCRYPTED password.
	Basic {
		email: String,
		password: Vec<u8>
	},

	// NOTE: OAuth implementation goes here.
}


/// Model representation for accounts database schema.
#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, PartialOrd, Clone)]
pub struct Account {
	/// The primary key for this model.
	id: i32,
	
	/// The account email.
	email: String,

	/// The account password.
	password: Vec<u8>,

	/// The account password salt to encrypt
	/// and decrypt with scrypt.
	password_salt: String
}


impl Account {
	/// Creates an account using the provided account creadentials,
	/// an [`AccountCreationResult`] is returned wrapping any
	/// client errors or the account itself.
	pub async fn create_account(
		connection: &PgPool,
		credentials: AccountCredentials
	) -> Result<AccountCreationResult, AccountError> {
		match credentials {
			AccountCredentials::Basic { email, password } => {
				let salt = SaltString::generate(&mut OsRng);
				let password_hash = Scrypt.hash_password(&password, &salt)?;

				let user = query_as(r"
					INSERT INTO accounts(email, password, salt)
					VALUES ($1, $2, $3)
					RETURNING *
				")
					.bind(email)
					.bind(password_hash.serialize().as_bytes())
					.bind(salt.as_str())
					.fetch_optional(connection)
					.await?;

				match user {
					Some(user) => Ok(AccountCreationResult::Created(user)),
					None => Ok(AccountCreationResult::AlreadyExists)
				}
			}
		}
	}

	/// Attempt to authenticate to an account using
	/// the provided credentials, if the credentials
	/// are incorrect Ok(None) is returned, if there
	/// is a server side error Err(..) is returned,
	/// otherwise Ok(Some(Self)).
	pub async fn get_by_auth(
		connection: &PgPool,
		credentials: AccountCredentials
	) -> Result<Option<Self>, AccountError> {
		match credentials {
			AccountCredentials::Basic { email, password } => {
				let account = query_as(r"
					SELECT * FROM accounts
					WHERE email = $1
					LIMIT 1
				")
					.bind(email)
					.fetch_optional(connection)
					.await?;

				let Some(account) = account else {
					return Ok(None);
				};

				
			}
		}
	}

	/// The primary key for this model.
	#[inline]
    pub fn id(&self) -> i32 {
        self.id
    }

	/// The account email.
	#[inline]
    pub fn email(&self) -> &str {
        &self.email
    }

	/// The account password.
	#[inline]
    pub fn password(&self) -> &[u8] {
        &self.password
    }
	/// The account password salt to encrypt
	/// and decrypt with scrypt.
	#[inline]
	pub fn password_salt(&self) -> &str {
        &self.password_salt
    }
}
