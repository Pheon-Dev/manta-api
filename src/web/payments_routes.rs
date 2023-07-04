use crate::ctx::Ctx;
use crate::model::{ModelController, Payment, PaymentForCreate};
use crate::Result;
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

pub fn routes(mc: ModelController) -> Router {
	Router::new()
		.route("/payments", post(create_payment).get(list_payments))
		.route("/payments/:id", delete(delete_payment))
		.with_state(mc)
}

// region:    --- REST Handlers
async fn create_payment(
	State(mc): State<ModelController>,
	ctx: Ctx,
	Json(payment_fc): Json<PaymentForCreate>,
) -> Result<Json<Payment>> {
	println!("->> {:<12} - create_payment", "HANDLER");

	let payment = mc.create_payment(ctx, payment_fc).await?;

	Ok(Json(payment))
}

async fn list_payments(
	State(mc): State<ModelController>,
	ctx: Ctx,
) -> Result<Json<Vec<Payment>>> {
	println!("->> {:<12} - list_payments", "HANDLER");

	let payments = mc.list_payments(ctx).await?;

	Ok(Json(payments))
}

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
