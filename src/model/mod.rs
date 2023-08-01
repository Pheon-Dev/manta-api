//! Model Layer
//!
//! Design:
//!
//! - The Model layer normalizes the application's data type structures and access.
//! - All application code data access must go through the Model layer.
//! - The `ModelManager` holds the internal states/resources needed by ModelControllers
//!   to access data. (e.g., db_pool, S3 client, redis client).
//! - Model Controllers (e.g., `paymentBmc`, `ProjectBmc`) implement
//!   CRUD and other data access methods on a given "entity" (e.g., `payment`, `Project`).
//!   (`Bmc` is short for Backend Model Controller).
//! - In frameworks like Axum, Tauri, `ModelManager` are typically used as App State.
//! - ModelManager are designed to be passed as an argument to all Model Controllers functions.
//!

// region:    --- Modules

pub mod account;
mod base;
pub mod card;
pub mod contact;
mod error;
pub mod payment;
mod store;
pub mod user;

pub use self::error::{Error, Result};

use crate::model::store::{new_db_pool, Db};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

impl ModelManager {
	/// Constructor
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;

		Ok(Self { db })
	}

	/// Return the sqlx db pool reference.
	/// (Only for the model layer)
	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
