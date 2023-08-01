use crate::ctx::Ctx;
use crate::model::base::DbBmc;
use crate::model::{base, ModelManager, Result};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlb::Fields;
use sqlx::FromRow;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Account {
	pub id: i64,

	pub balance: String,
	pub username: String,
	pub email: String,
	pub aid: String,
	pub cookie: String,

	// -- Timestamps
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Deserialize, Fields)]
pub struct AccountForCreate {
	pub balance: String,
	pub username: String,
	pub email: String,
	pub aid: String,
	pub cookie: String,
}

#[derive(Deserialize, Fields)]
pub struct AccountForUpdate {
	pub balance: Option<String>,
	pub username: Option<String>,
	pub email: Option<String>,
	pub aid: Option<String>,
	pub cookie: Option<String>,
}
// endregion: --- Account Types

pub struct AccountBmc;

impl DbBmc for AccountBmc {
	const TABLE: &'static str = "account";
	const HAS_TIMESTAMPS: bool = true;
}

impl AccountBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		account_c: AccountForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, account_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Account> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Account>> {
		base::list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		account_u: AccountForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, account_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
