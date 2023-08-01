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
pub struct Contact {
	pub id: i64,

	pub username: String,
	pub ref_id: String,
	pub association: String,
	pub name: String,
	pub email: String,

	// -- Timestamps
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Deserialize, Fields)]
pub struct ContactForCreate {
	pub username: String,
	pub ref_id: String,
	pub association: String,
	pub name: String,
	pub email: String,
}

#[derive(Deserialize, Fields)]
pub struct ContactForUpdate {
	pub username: Option<String>,
	pub ref_id: Option<String>,
	pub association: Option<String>,
	pub name: Option<String>,
	pub email: Option<String>,
}
// endregion: --- Contact Types

pub struct ContactBmc;

impl DbBmc for ContactBmc {
	const TABLE: &'static str = "contact";
	const HAS_TIMESTAMPS: bool = true;
}

impl ContactBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		contact_c: ContactForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, contact_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Contact> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Contact>> {
		base::list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		contact_u: ContactForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, contact_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
