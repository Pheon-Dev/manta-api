use crate::ctx::Ctx;
use crate::model::base::DbBmc;
use crate::model::{base, ModelManager, Result};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlb::Fields;
use sqlx::FromRow;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

// region:    --- Send Types
#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Send {
	pub id: i64,

	pub amount: String,

	// -- Timestamps
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Deserialize, Fields)]
pub struct SendForCreate {
	pub amount: String,
}

#[derive(Deserialize, Fields)]
pub struct SendForUpdate {
	pub amount: Option<String>,
}
// endregion: --- Send Types

pub struct SendBmc;

impl DbBmc for SendBmc {
	const TABLE: &'static str = "send";
	const HAS_TIMESTAMPS: bool = true;
}

impl SendBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		send_c: SendForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, send_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Send> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Send>> {
		base::list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		send_u: SendForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, send_u).await
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
		let fx_amount = "test_model_send_create_basic";

		// -- Exec
		let send_c = SendForCreate { amount: fx_amount.to_string() };
		let id = SendBmc::create(&ctx, &mm, send_c).await?;

		// -- Check
		let send = SendBmc::get(&ctx, &mm, id).await?;
		assert_eq!(send.amount, fx_amount);

		// -- Clean
		SendBmc::delete(&ctx, &mm, id).await?;

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
		let res = SendBmc::get(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(res, Err(Error::EntityNotFound { entity: "send", id: 100 })),
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
		let fx_amounts = &["test_list_ok 01", "test_list_ok 02"];
		_dev_utils::seed_sends(&ctx, &mm, fx_amounts).await?;

		// -- Exec
		let sends = SendBmc::list(&ctx, &mm).await?;

		// -- Check
		let sends: Vec<Send> = sends
			.into_iter()
			.filter(|t| t.amount.starts_with("test_list_ok"))
			.collect();
		assert_eq!(sends.len(), 2, "number of seeded sends.");

		// -- Clean
		for send in sends.iter() {
			SendBmc::delete(&ctx, &mm, send.id).await?;
		}

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_update_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_amount = "test_update_ok - send 01";
		let fx_amount_new = "test_update_ok - send 01 - new";
		let fx_send = _dev_utils::seed_sends(&ctx, &mm, &[fx_amount])
			.await?
			.remove(0);

		// -- Exec
		SendBmc::update(
			&ctx,
			&mm,
			fx_send.id,
			SendForUpdate { amount: Some(fx_amount_new.to_string()) },
		)
		.await?;

		// -- Check
		let send = SendBmc::get(&ctx, &mm, fx_send.id).await?;
		assert_eq!(send.amount, fx_amount_new);

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
		let res = SendBmc::delete(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(res, Err(Error::EntityNotFound { entity: "send", id: 100 })),
			"EntityNotFound not matching"
		);

		Ok(())
	}
}
// endregion: --- Tests
