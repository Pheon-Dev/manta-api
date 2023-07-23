use crate::ctx::Ctx;
use crate::model::payment::{
	Payment, PaymentBmc, PaymentForCreate, PaymentForUpdate,
};
use crate::model::ModelManager;
use crate::web::rpc::{DataResult, ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::web::Result;

#[utoipa::path(
    post,
    path = "/api/rpc",
    request_body = ParamsForCreate,
    responses(
        (status = 200, description = "Sent Successfully", body = [Payment]),
        (status = 404, description = "Failed to payment"),
    )
)]
pub async fn create_payment(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<PaymentForCreate>,
) -> Result<DataResult<Payment>> {
	let ParamsForCreate { data } = params;

	let id = PaymentBmc::create(&ctx, &mm, data).await?;
	let payment = PaymentBmc::get(&ctx, &mm, id).await?;

	Ok(DataResult::new(payment))
}

#[utoipa::path(
    post,
    path = "/api/rpc",
    responses(
        (status = 200, description = "Sent Successfully", body = [Payment]),
        (status = 404, description = "Failed to payment"),
    )
)]
pub async fn list_payments(
	ctx: Ctx,
	mm: ModelManager,
) -> Result<DataResult<Vec<Payment>>> {
	let payments = PaymentBmc::list(&ctx, &mm).await?;

	Ok(DataResult::new(payments))
}

#[utoipa::path(
    post,
    path = "/api/rpc",
    request_body = ParamsForUpdate,
    responses(
        (status = 200, description = "Updated Successfully", body = Payment),
        (status = 404, description = "Failed to Update"),
    )
)]
pub async fn update_payment(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<PaymentForUpdate>,
) -> Result<DataResult<Payment>> {
	let ParamsForUpdate { id, data } = params;

	PaymentBmc::update(&ctx, &mm, id, data).await?;

	let payment = PaymentBmc::get(&ctx, &mm, id).await?;

	Ok(DataResult::new(payment))
}

#[utoipa::path(
    post,
    path = "/api/rpc",
    // request_body = Payment,
    responses(
        (status = 200, description = "Deleted Successfully", body = Payment),
        (status = 404, description = "Failed to Delete"),
    )
)]
pub async fn delete_payment(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<DataResult<Payment>> {
	let ParamsIded { id } = params;

	let payment = PaymentBmc::get(&ctx, &mm, id).await?;
	PaymentBmc::delete(&ctx, &mm, id).await?;

	Ok(DataResult::new(payment))
}
