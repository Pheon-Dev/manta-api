//! Simplistic Model Layer
//! (with mock-store layer)

use crate::ctx::Ctx;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use utoipa::ToSchema;

// region:    --- Payment Types
#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Payment {
	pub id: u64,
	pub cid: u64, // creator user_id
	pub amount: String,
    pub sender: String,
    pub receiver: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct PaymentForCreate {
	pub amount: String,
    pub sender: String,
    pub receiver: String,
    pub description: String,
    pub completed: bool,
}
// endregion: --- Payment Types

// region:    --- Model Controller
#[derive(Clone)]
pub struct ModelController {
	payments_store: Arc<Mutex<Vec<Option<Payment>>>>,
}

// Constructor
impl ModelController {
	pub async fn new() -> Result<Self> {
		Ok(Self {
			payments_store: Arc::default(),
		})
	}
}

// CRUD Implementation
impl ModelController {
	pub async fn create_payment(
		&self,
		ctx: Ctx,
		payment_fc: PaymentForCreate,
	) -> Result<Payment> {
		let mut store = self.payments_store.lock().unwrap();

		let id = store.len() as u64;
		let payment = Payment {
			id,
			cid: ctx.user_id(),
			amount: payment_fc.amount,
            sender: payment_fc.sender,
            receiver: payment_fc.receiver,
            description: payment_fc.description,
            completed: payment_fc.completed,
		};
		store.push(Some(payment.clone()));

		Ok(payment)
	}

	pub async fn list_payments(&self, _ctx: Ctx) -> Result<Vec<Payment>> {
		let store = self.payments_store.lock().unwrap();

		let payments = store.iter().filter_map(|t| t.clone()).collect();

		Ok(payments)
	}

    pub async fn details_payment(&self, _ctx: Ctx, id: u64) -> Result<Payment> {
        let store = self.payments_store.lock().unwrap();
        let payment = store.get(id as usize).and_then(|t| t.clone());
        payment.ok_or(Error::PaymentNotFound { id })
    }

	pub async fn delete_payment(&self, _ctx: Ctx, id: u64) -> Result<Payment> {
		let mut store = self.payments_store.lock().unwrap();
		let payment = store.get_mut(id as usize).and_then(|t| t.take());
		payment.ok_or(Error::PaymentDeleteFailIdNotFound { id })
	}
}

// endregion: --- Model Controller
