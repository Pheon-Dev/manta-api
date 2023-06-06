use crate::ctx::Ctx;
use crate::model::send_req::{SendReq, SendReqForCreate, SendReqBmc, SendReqForUpdate};
use crate::model::ModelManager;
use crate::web::rpc::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::web::Result;

pub async fn create_send_req(ctx: Ctx, mm: ModelManager, params: ParamsForCreate<SendReqForCreate>) -> Result<SendReq> {
    let ParamsForCreate { data } = params;

    let id = SendReqBmc::create(&ctx, &mm, data).await?;
    let send_req = SendReqBmc::get(&ctx, &mm, id).await?;
    Ok(send_req)
}

pub async fn list_send_reqs(mm: ModelManager, ctx: Ctx) -> Result<Vec<SendReq>> {
    let send_reqs = SendReqBmc::list(&ctx, &mm).await?;
    Ok(send_reqs)
}

pub async fn update_send_req(ctx: Ctx, mm: ModelManager, params: ParamsForUpdate<SendReqForUpdate>,) -> Result<SendReq> {
    let ParamsForUpdate { id, data } = params;

    SendReqBmc::update(&ctx, &mm, id, data).await?;

    let send_req = SendReqBmc::get(&ctx, &mm, id).await?;

    Ok(send_req)
}

pub async fn delete_send_req(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<SendReq> {
    let ParamsIded { id } = params;

    let send_req = SendReqBmc::get(&ctx, &mm, id).await?;
    SendReqBmc::delete(&ctx, &mm, id).await?;

    Ok(send_req)
}
