// region:    --- Modules

mod dev_db;

use crate::ctx::Ctx;
use crate::model::account::{Account, AccountBmc, AccountForCreate};
use crate::model::card::{Card, CardBmc, CardForCreate};
use crate::model::contact::{Contact, ContactBmc, ContactForCreate};
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

pub async fn seed_contacts(
	ctx: &Ctx,
	mm: &ModelManager,
	ref_ids: &[&str],
	associations: &[&str],
	names: &[&str],
	emails: &[&str],
	usernames: &[&str],
) -> model::Result<Vec<Contact>> {
	let mut contacts = Vec::new();

	for username in usernames {
		for ref_id in ref_ids {
			for name in names {
				for association in associations {
					for email in emails {
						let id = ContactBmc::create(
							ctx,
							mm,
							ContactForCreate {
								username: username.to_string(),
								ref_id: ref_id.to_string(),
								name: name.to_string(),
								association: association.to_string(),
								email: email.to_string(),
							},
						)
						.await?;
						let contact = ContactBmc::get(ctx, mm, id).await?;

						contacts.push(contact);
					}
				}
			}
		}
	}

	Ok(contacts)
}

pub async fn seed_cards(
	ctx: &Ctx,
	mm: &ModelManager,
	cbalances: &[&str],
	cnumbers: &[&str],
	ctypes: &[&str],
	cdescriptions: &[&str],
	caccounts: &[&str],
	cvalids: &[&str],
	cvvs: &[&str],
	cnames: &[&str],
) -> model::Result<Vec<Card>> {
	let mut cards = Vec::new();

	for cname in cnames {
		for cbalance in cbalances {
			for ctype in ctypes {
				for cnumber in cnumbers {
					for cvalid in cvalids {
						for cvv in cvvs {
							for caccount in caccounts {
								for cdescription in cdescriptions {
									let id = CardBmc::create(
										ctx,
										mm,
										CardForCreate {
											cname: cname.to_string(),
											cbalance: cbalance.to_string(),
											ctype: ctype.to_string(),
											cnumber: cnumber.to_string(),
											caccount: caccount.to_string(),
											cvv: cvv.to_string(),
											cvalid: cvalid.to_string(),
											cdescription: cdescription.to_string(),
										},
									)
									.await?;
									let card = CardBmc::get(ctx, mm, id).await?;

									cards.push(card);
								}
							}
						}
					}
				}
			}
		}
	}

	Ok(cards)
}

pub async fn seed_accounts(
	ctx: &Ctx,
	mm: &ModelManager,
	usernames: &[&str],
	emails: &[&str],
	aids: &[&str],
	cookies: &[&str],
	balances: &[&str],
) -> model::Result<Vec<Account>> {
	let mut accounts = Vec::new();

	for balance in balances {
		for username in usernames {
			for aid in aids {
				for email in emails {
					for cookie in cookies {
						let id = AccountBmc::create(
							ctx,
							mm,
							AccountForCreate {
								balance: balance.to_string(),
								username: username.to_string(),
								aid: aid.to_string(),
								email: email.to_string(),
								cookie: cookie.to_string(),
							},
						)
						.await?;
						let account = AccountBmc::get(ctx, mm, id).await?;

						accounts.push(account);
					}
				}
			}
		}
	}

	Ok(accounts)
}
