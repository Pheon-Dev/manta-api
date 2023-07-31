// region:    --- Modules

mod dev_db;

use crate::ctx::Ctx;
use crate::model::payment::{Payment, PaymentBmc, PaymentForCreate};
use crate::model::{self, ModelManager};
use tokio::sync::OnceCell;
use tracing::info;

// endregion: --- Modules

// Initialize environment for local development.
// (for early development, called from `main()`)
pub async fn init_dev() {
	static INIT: OnceCell<()> = OnceCell::const_new();

	INIT.get_or_init(|| async {
		info!("{:<12} - init_dev()", "FOR-DEV-ONLY");

		dev_db::init_dev_db().await.unwrap();
	})
	.await;
}

/// Initialize test environment.
pub async fn init_test() -> ModelManager {
	static INIT: OnceCell<ModelManager> = OnceCell::const_new();

	let mm = INIT
		.get_or_init(|| async {
			init_dev().await;
			ModelManager::new().await.unwrap()
		})
		.await;

	mm.clone()
}

pub async fn seed_payments(
	ctx: &Ctx,
	mm: &ModelManager,
	amounts: &[&str],
	receivers: &[&str],
	senders: &[&str],
	descriptions: &[&str],
) -> model::Result<Vec<Payment>> {
	let mut payments = Vec::new();

	for amount in amounts {
		for sender in senders {
			for receiver in receivers {
				for description in descriptions {
					let id = PaymentBmc::create(
						ctx,
						mm,
						PaymentForCreate {
							amount: amount.to_string(),
							sender: sender.to_string(),
							receiver: receiver.to_string(),
							description: description.to_string(),
						},
					)
					.await?;
					let payment = PaymentBmc::get(ctx, mm, id).await?;

					payments.push(payment);
				}
			}
		}
	}

	Ok(payments)
}
