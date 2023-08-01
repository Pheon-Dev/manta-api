//! Simplistic Model Layer
//! (with mock-store layer)

use crate::ctx::Ctx;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use utoipa::ToSchema;

// region:    --- Account Types
#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Account {
	pub id: u64,
	pub cid: u64, // creator user_id
	pub balance: String,
	pub aid: String,
	pub cookie: String,
	pub email: String,
	pub username: String,
}

#[derive(Deserialize, ToSchema)]
pub struct AccountForCreate {
	pub balance: String,
	pub aid: String,
	pub cookie: String,
	pub email: String,
	pub username: String,
}
// endregion: --- Account Types

use crate::model::store::{new_db_pool, Db};

// region:    --- Model Controller
#[derive(Clone)]
pub struct ModelController {
	accounts_store: Arc<Mutex<Vec<Option<Account>>>>,
	// db: Db,
}

// Constructor
impl ModelController {
	pub async fn new() -> Result<Self> {
		// let db = new_db_pool().await?;
		// Ok(Self { db })
		Ok(Self {
			accounts_store: Arc::default(),
		})
	}
	// pub(in crate::model) fn db(&self) -> &Db {
	// 	&self.db
	// }
}

// CRUD Implementation
impl ModelController {
	pub async fn create_account(
		&self,
		ctx: Ctx,
		account_fc: AccountForCreate,
	) -> Result<Account> {
		let mut store = self.accounts_store.lock().unwrap();

		let id = store.len() as u64;
		let account = Account {
			id,
			cid: ctx.user_id(),
			balance: account_fc.balance,
			aid: account_fc.aid,
			cookie: account_fc.cookie,
			email: account_fc.email,
			username: account_fc.username,
		};
		store.push(Some(account.clone()));

		Ok(account)
	}

	pub async fn list_accounts(&self, _ctx: Ctx) -> Result<Vec<Account>> {
		let store = self.accounts_store.lock().unwrap();

		let accounts = store.iter().filter_map(|t| t.clone()).collect();

		Ok(accounts)
	}

	pub async fn details_account(&self, _ctx: Ctx, id: u64) -> Result<Account> {
		let store = self.accounts_store.lock().unwrap();
		let account = store.get(id as usize).and_then(|t| t.clone());
		account.ok_or(Error::AccountNotFound { id })
	}

	pub async fn delete_account(&self, _ctx: Ctx, id: u64) -> Result<Account> {
		let mut store = self.accounts_store.lock().unwrap();
		let account = store.get_mut(id as usize).and_then(|t| t.take());
		account.ok_or(Error::AccountDeleteFailIdNotFound { id })
	}
}

// endregion: --- Model Controller
