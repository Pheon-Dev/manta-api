//! Simplistic Model Layer
//! (with mock-store layer)

use crate::ctx::Ctx;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use utoipa::ToSchema;

// region:    --- Contact Types
#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Contact {
	pub id: u64,
	pub cid: u64, // creator user_id
	pub username: String,
	pub ref_id: String,
	pub association: String,
	pub email: String,
	pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ContactForCreate {
	pub username: String,
	pub ref_id: String,
	pub association: String,
	pub email: String,
	pub name: String,
}
// endregion: --- Contact Types

use crate::model::store::{new_db_pool, Db};

// region:    --- Model Controller
#[derive(Clone)]
pub struct ModelController {
	contacts_store: Arc<Mutex<Vec<Option<Contact>>>>,
	// db: Db,
}

// Constructor
impl ModelController {
	pub async fn new() -> Result<Self> {
		// let db = new_db_pool().await?;
		// Ok(Self { db })
		Ok(Self {
			contacts_store: Arc::default(),
		})
	}
	// pub(in crate::model) fn db(&self) -> &Db {
	// 	&self.db
	// }
}

// CRUD Implementation
impl ModelController {
	pub async fn create_contact(
		&self,
		ctx: Ctx,
		contact_fc: ContactForCreate,
	) -> Result<Contact> {
		let mut store = self.contacts_store.lock().unwrap();

		let id = store.len() as u64;
		let contact = Contact {
			id,
			cid: ctx.user_id(),
			username: contact_fc.username,
			ref_id: contact_fc.ref_id,
			association: contact_fc.association,
			email: contact_fc.email,
			name: contact_fc.name,
		};
		store.push(Some(contact.clone()));

		Ok(contact)
	}

	pub async fn list_contacts(&self, _ctx: Ctx) -> Result<Vec<Contact>> {
		let store = self.contacts_store.lock().unwrap();

		let contacts = store.iter().filter_map(|t| t.clone()).collect();

		Ok(contacts)
	}

	pub async fn details_contact(&self, _ctx: Ctx, id: u64) -> Result<Contact> {
		let store = self.contacts_store.lock().unwrap();
		let contact = store.get(id as usize).and_then(|t| t.clone());
		contact.ok_or(Error::ContactNotFound { id })
	}

	pub async fn delete_contact(&self, _ctx: Ctx, id: u64) -> Result<Contact> {
		let mut store = self.contacts_store.lock().unwrap();
		let contact = store.get_mut(id as usize).and_then(|t| t.take());
		contact.ok_or(Error::ContactDeleteFailIdNotFound { id })
	}
}

// endregion: --- Model Controller
