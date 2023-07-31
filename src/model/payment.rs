use crate::ctx::Ctx;
use crate::model::base::DbBmc;
use crate::model::{base, ModelManager, Result};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlb::Fields;
use sqlx::FromRow;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

// region:    --- Payment Types B061-7360
#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Payment {
	pub id: i64,

	pub amount: String,
	pub sender: String,
	pub receiver: String,
	pub description: String,

	// -- Timestamps
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Deserialize, Fields)]
pub struct PaymentForCreate {
	pub amount: String,
	pub sender: String,
	pub receiver: String,
	pub description: String,
}

#[derive(Deserialize, Fields)]
pub struct PaymentForUpdate {
	pub amount: Option<String>,
	pub sender: Option<String>,
	pub receiver: Option<String>,
	pub description: Option<String>,
}
// endregion: --- Payment Types

pub struct PaymentBmc;

impl DbBmc for PaymentBmc {
	const TABLE: &'static str = "payment";
	const HAS_TIMESTAMPS: bool = true;
}

impl PaymentBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		payment_c: PaymentForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, payment_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Payment> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Payment>> {
		base::list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		payment_u: PaymentForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, payment_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}

// region:    --- Tests
#[cfg(test)]
mod tests {
	use super::*;
	use crate::_dev_utils;
	use crate::model::Error;
	use anyhow::Result;
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_amount = "100";
		let fx_sender = "Jane";
		let fx_receiver = "Jenny";
		let fx_description = "School Fees";

		// -- Exec
		let payment_c = PaymentForCreate {
			amount: fx_amount.to_string(),
			sender: fx_sender.to_string(),
			receiver: fx_receiver.to_string(),
			description: fx_description.to_string(),
		};
		let id = PaymentBmc::create(&ctx, &mm, payment_c).await?;

		// -- Check
		let payment = PaymentBmc::get(&ctx, &mm, id).await?;
		assert_eq!(payment.amount, fx_amount);

		// -- Clean
		PaymentBmc::delete(&ctx, &mm, id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_get_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Exec
		let res = PaymentBmc::get(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(
				res,
				Err(Error::EntityNotFound { entity: "payment", id: 100 })
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_amounts = &["200", "2500"];
		let fx_senders = &["Jake Smith", "Mary Beth"];
		let fx_receivers = &["Paul Drake", "Peter Pan"];
		let fx_descriptions = &["Lunch", "Shopping"];
		_dev_utils::seed_payments(
			&ctx,
			&mm,
			fx_amounts,
			fx_senders,
			fx_receivers,
			fx_descriptions,
		)
		.await?;

		// -- Exec
		let payments = PaymentBmc::list(&ctx, &mm).await?;

		// -- Check
		let payments: Vec<Payment> = payments
			.into_iter()
			.filter(|t| t.amount.starts_with("test_list_ok"))
			.collect();
		assert_eq!(payments.len(), 2, "number of seeded payments.");

		// -- Clean
		for payment in payments.iter() {
			PaymentBmc::delete(&ctx, &mm, payment.id).await?;
		}

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_update_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_amount = "3003";
		let fx_sender = "3003";
		let fx_receiver = "3000";
		let fx_description = "3003";

		let fx_amount_new = "3000";
		let fx_sender_new = "3000";
		let fx_receiver_new = "3000";
		let fx_description_new = "3000";

		let fx_payment = _dev_utils::seed_payments(
			&ctx,
			&mm,
			&[fx_amount],
			&[fx_sender],
			&[fx_receiver],
			&[fx_description],
		)
		.await?
		.remove(0);

		// -- Exec
		PaymentBmc::update(
			&ctx,
			&mm,
			fx_payment.id,
			PaymentForUpdate {
				amount: Some(fx_amount_new.to_string()),
				sender: Some(fx_sender_new.to_string()),
				receiver: Some(fx_receiver_new.to_string()),
				description: Some(fx_description_new.to_string()),
			},
		)
		.await?;

		// -- Check
		let payment = PaymentBmc::get(&ctx, &mm, fx_payment.id).await?;
		assert_eq!(payment.amount, fx_amount_new);

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_delete_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Exec
		let res = PaymentBmc::delete(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(
				res,
				Err(Error::EntityNotFound { entity: "payment", id: 100 })
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}
}
// endregion: --- Tests
