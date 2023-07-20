mod dev_db;

use crate::ctx::Ctx;
use crate::model::{self, ModelController};
use crate::model::{Payment, PaymentForCreate};
use tokio::sync::OnceCell;
use tracing::info;

pub async fn init_dev() {
	static INIT: OnceCell<()> = OnceCell::const_new();

	INIT.get_or_init(|| async {
		info!("{:<12} - init_dev()", "FOR-DEV-ONLY");

		dev_db::init_dev_db().await.unwrap();
	})
	.await;
}

pub async fn init_test() -> ModelController {
	static INIT: OnceCell<ModelController> = OnceCell::const_new();

	let mc = INIT
		.get_or_init(|| async {
			init_dev().await;
			ModelController::new().await.unwrap()
		})
		.await;
	mc.clone()
}

// pub async fn seed_payments(
// 	ctx: &Ctx,
// 	mc: &ModelController,
// 	payments: Vec<PaymentForCreate>,
// ) {
// 	let mut payments = Vec::new();
// 	for payment in payments {
// 		payments.push(mc.create_payment(ctx, payment).await?);
// 	}
// 	Ok(payments)
// }