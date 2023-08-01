//! Simplistic Model Layer
//! (with mock-store layer)

use crate::ctx::Ctx;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use utoipa::ToSchema;

// region:    --- Card Types
#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Card {
	pub id: u64,
	pub cid: u64, // creator user_id
	pub name: String,
	pub balance: String,
	pub number: String,
	pub description: String,
	pub account: String,
	pub valid: String,
	pub cvv: String,
	pub card_type: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CardForCreate {
	pub name: String,
	pub balance: String,
	pub number: String,
	pub description: String,
	pub account: String,
	pub valid: String,
	pub cvv: String,
	pub card_type: String,
}
// endregion: --- Card Types

use crate::model::store::{new_db_pool, Db};

// region:    --- Model Controller
#[derive(Clone)]
pub struct ModelController {
	cards_store: Arc<Mutex<Vec<Option<Card>>>>,
	// db: Db,
}

// Constructor
impl ModelController {
	pub async fn new() -> Result<Self> {
		// let db = new_db_pool().await?;
		// Ok(Self { db })
		Ok(Self {
			cards_store: Arc::default(),
		})
	}
	// pub(in crate::model) fn db(&self) -> &Db {
	// 	&self.db
	// }
}

// CRUD Implementation
impl ModelController {
	pub async fn create_card(
		&self,
		ctx: Ctx,
		card_fc: CardForCreate,
	) -> Result<Card> {
		let mut store = self.cards_store.lock().unwrap();

		let id = store.len() as u64;
		let card = Card {
			id,
			cid: ctx.user_id(),
			name: card_fc.name,
			balance: card_fc.balance,
			number: card_fc.number,
			description: card_fc.description,
			account: card_fc.account,
			valid: card_fc.valid,
			cvv: card_fc.cvv,
			card_type: card_fc.card_type,
		};
		store.push(Some(card.clone()));

		Ok(card)
	}

	pub async fn list_cards(&self, _ctx: Ctx) -> Result<Vec<Card>> {
		let store = self.cards_store.lock().unwrap();

		let cards = store.iter().filter_map(|t| t.clone()).collect();

		Ok(cards)
	}

	pub async fn details_card(&self, _ctx: Ctx, id: u64) -> Result<Card> {
		let store = self.cards_store.lock().unwrap();
		let card = store.get(id as usize).and_then(|t| t.clone());
		card.ok_or(Error::CardNotFound { id })
	}

	pub async fn delete_card(&self, _ctx: Ctx, id: u64) -> Result<Card> {
		let mut store = self.cards_store.lock().unwrap();
		let card = store.get_mut(id as usize).and_then(|t| t.take());
		card.ok_or(Error::CardDeleteFailIdNotFound { id })
	}
}

// endregion: --- Model Controller
