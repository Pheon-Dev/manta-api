use crate::ctx::Ctx;
use crate::model::base::DbBmc;
use crate::model::{base, ModelManager, Result};

use serde_with::serde_as;
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

// region: --- SendReq Types
#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct SendReq {
    pub id: i64,

    pub amount: String,
    pub currency: String,
    pub receiver: String,
    pub description: String,

    pub cid: i64,
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
    pub mid: i64,
    #[serde_as(as = "Rfc3339")]
    pub mtime: OffsetDateTime,
}

#[derive(Deserialize, Fields)]
pub struct SendReqForCreate {
    pub amount: String,
    pub currency: String,
    pub receiver: String,
    pub description: String,
}

#[derive(Deserialize, Fields)]
pub struct SendReqForUpdate {
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub receiver: Option<String>,
    pub description: Option<String>,
}
// endregion: --- SendReq Types

pub struct SendReqBmc;

impl DbBmc for SendReqBmc {
    const TABLE: &'static str = "send_req";
    const HAS_TIMESTAMPS: bool = true;
}

impl SendReqBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        send_req_c: SendReqForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, send_req_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<SendReq> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<SendReq>> {
        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn update(ctx: &Ctx, mm: &ModelManager, id: i64, send_req_u: SendReqForUpdate,) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, send_req_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

// TODO: Tests
