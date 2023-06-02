use crate::ctx::Ctx;
use crate::model::{Error, ModelManager, Result};

use sqlb::HasFields;

pub trait DbBmc {
    const TABLE: &'static str;
    const HAS_TIMESTAMPS: bool;
}

pub async fn create<MC, E>(ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
where
MC: DbBmc,
    E: HasFields, {
    // let db = mm.db();
    // let (id, ) = sqlb::insert()
    //     .table(MC::TABLE)
    //     .data(fields)
    //     .returning(&["id"])
    //     .fetch_one::<_, (i64)>(db)
    //     .await?;
    // Ok(id)
    Ok(89)
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {Ok(())}

pub async fn list<MC, E>() -> Result<()> {Ok(())}

pub async fn update<MC, E>() -> Result<()> {Ok(())}

pub async fn delete<MC, E>() -> Result<()> {Ok(())}
