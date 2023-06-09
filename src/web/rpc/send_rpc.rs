use crate::ctx::Ctx;
use crate::model::send::{Send, SendBmc, SendForCreate, SendForUpdate};
use crate::model::ModelManager;
use crate::web::rpc::{DataResult, ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::web::Result;

#[utoipa::path(
    post,
    path = "/api/rpc",
    request_body = ParamsForCreate,
    responses(
        (status = 200, description = "Sent Successfully", body = [Send]),
        (status = 404, description = "Failed to Send"),
    )
)]
pub async fn create_send(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<SendForCreate>,
) -> Result<DataResult<Send>> {
	let ParamsForCreate { data } = params;

	let id = SendBmc::create(&ctx, &mm, data).await?;
	let send = SendBmc::get(&ctx, &mm, id).await?;

	Ok(DataResult::new(send))
}

#[utoipa::path(
    post,
    path = "/api/rpc",
    responses(
        (status = 200, description = "Sent Successfully", body = [Send]),
        (status = 404, description = "Failed to Send"),
    )
)]
pub async fn list_sends(
	ctx: Ctx,
	mm: ModelManager,
) -> Result<DataResult<Vec<Send>>> {
	let sends = SendBmc::list(&ctx, &mm).await?;

	Ok(DataResult::new(sends))
}

#[utoipa::path(
    post,
    path = "/api/rpc",
    request_body = ParamsForUpdate,
    responses(
        (status = 200, description = "Updated Successfully", body = Send),
        (status = 404, description = "Failed to Update"),
    )
)]
pub async fn update_send(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<SendForUpdate>,
) -> Result<DataResult<Send>> {
	let ParamsForUpdate { id, data } = params;

	SendBmc::update(&ctx, &mm, id, data).await?;

	let send = SendBmc::get(&ctx, &mm, id).await?;

	Ok(DataResult::new(send))
}

#[utoipa::path(
    post,
    path = "/api/rpc",
    request_body = ParamsIded,
    responses(
        (status = 200, description = "Deleted Successfully", body = Send),
        (status = 404, description = "Failed to Delete"),
    )
)]
pub async fn delete_send(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<DataResult<Send>> {
	let ParamsIded { id } = params;

	let send = SendBmc::get(&ctx, &mm, id).await?;
	SendBmc::delete(&ctx, &mm, id).await?;

	Ok(DataResult::new(send))
}
