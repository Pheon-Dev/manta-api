#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	// hc.do_get("/src/main.rs").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"password": "welcome"
		}),
	);
	req_login.await?.print().await?;

	let req_create_payment = hc.do_post(
		"/api/payments",
		json!({
			"amount": "5000",
			"sender": "john",
			"receiver": "jane",
			"description": "shopping",
		}),
	);
	req_create_payment.await?.print().await?;

	hc.do_get("/api/payments").await?.print().await?;

	let req_create_card = hc.do_post(
		"/api/cards",
		json!({
			"name": "John Paul",
			"balance": "32000",
			"number": "4632-6343-7541-0674",
			"card_type": "VISA",
			"cvv": "058",
			"valid": "04/25",
			"description": "M-PESA Global Pay Card",
			"account": "Debit",
		}),
	);
	req_create_card.await?.print().await?;

	hc.do_get("/api/cards").await?.print().await?;

	Ok(())
}
