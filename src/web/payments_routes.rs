use crate::ctx::Ctx;
use crate::model::{ModelController, Payment, PaymentForCreate};
use crate::Result;

use axum::extract::{Path, State};
use axum::routing::{delete, post, get};
use axum::{Json, Router};

pub fn routes(mc: ModelController) -> Router {
	Router::new()
		.route("/payments", post(create_payment).get(list_payments))
		.route("/payments/delete/:id", delete(delete_payment))
		.route("/payments/details/:id", get(details_payment))
		.with_state(mc)
}

// region:    --- REST Handlers
#[utoipa::path(
    post,
    path = "/api/payments",
    request_body = PaymentForCreate,
    responses((
        status = 200,
        body = [Payment],
        description = "Payment",
    ), ( status = 404, description = "Payment Not Found"))
)]
async fn create_payment(
	State(mc): State<ModelController>,
	ctx: Ctx,
	Json(payment_fc): Json<PaymentForCreate>,
) -> Result<Json<Payment>> {
	println!("->> {:<12} - create_payment", "HANDLER");

	let payment = mc.create_payment(ctx, payment_fc).await?;

	Ok(Json(payment))
}

#[utoipa::path(
    get,
    path = "/api/payments",
    responses((
        status = 200,
        body = [Payment],
        description = "Payments List",
    ), ( status = 404, description = "Payment List Not Found"))
)]
async fn list_payments(
	State(mc): State<ModelController>,
	ctx: Ctx,
) -> Result<Json<Vec<Payment>>> {
	println!("->> {:<12} - list_payments", "HANDLER");

	let payments = mc.list_payments(ctx).await?;

	Ok(Json(payments))
}

#[utoipa::path(
    get,
    path = "/api/payments/details/{id}",
    params(("id" = u64, Path, description = "Payment ID")),
    responses((
        status = 200,
        body = Payment,
        description = "Payment Details",
    ), ( status = 404, description = "Payment Details Not Found"))
)]
async fn details_payment(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Payment>>> {
    println!("->> {:<12} - payment_details", "HANDLER");
    let payments = mc.details_payment(ctx).await?;
    Ok(Json(payments))
}

#[utoipa::path(
    delete,
    path = "/api/payments/delete/{id}",
    params(("id" = u64, Path, description = "Payment ID")),
    responses((
        status = 200,
        description = "Payment Deleted",
    ), (status = 404, description = "Failed to Delete Payment"))
)]
async fn delete_payment(
	State(mc): State<ModelController>,
	ctx: Ctx,
	Path(id): Path<u64>,
) -> Result<Json<Payment>> {
	println!(">>> {:<12} - delete_payment", "HANDLER");

	let payment = mc.delete_payment(ctx, id).await?;

	Ok(Json(payment))
}
// endregion: --- REST Handlers
