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
pub struct Card {
	pub id: i64,

	pub cname: String,
	pub cbalance: String,
	pub cnumber: String,
	pub ctype: String,
	pub caccount: String,
	pub cvalid: String,
	pub cvv: String,
	pub cdescription: String,

	// -- Timestamps
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Deserialize, Fields)]
pub struct CardForCreate {
	pub cname: String,
	pub cbalance: String,
	pub cnumber: String,
	pub ctype: String,
	pub caccount: String,
	pub cvalid: String,
	pub cvv: String,
	pub cdescription: String,
}

#[derive(Deserialize, Fields)]
pub struct CardForUpdate {
	pub amount: Option<String>,
	pub sender: Option<String>,
	pub receiver: Option<String>,
	pub description: Option<String>,
}
// endregion: --- Card Types

pub struct CardBmc;

impl DbBmc for CardBmc {
	const TABLE: &'static str = "card";
	const HAS_TIMESTAMPS: bool = true;
}

impl CardBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		card_c: CardForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, card_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Card> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Card>> {
		base::list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		card_u: CardForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, card_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
