#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	// hc.do_get("/index.html").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "janedoe",
			"password": "welcome"
		}),
	);
	req_login.await?.print().await?;

	let req_create_payment = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_payment",
			"params": {
				"data": {
					"amount": "10200",
					"sender": "Jane Doe",
					"receiver": "John Paul",
					"description": "School Fees and Lunch",
				}
			}
		}),
	);
	req_create_payment.await?.print().await?;

	let req_update_payment = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_payment",
			"params": {
				"id": 1000, // Hardcode the send id.
				"data": {
					"amount": "200"
				}
			}
		}),
	);
	// req_update_payment.await?.print().await?;

	let req_delete_payment = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "delete_payment",
			"params": {
				"id": 1001 // Harcode the send id
			}
		}),
	);
	// req_delete_payment.await?.print().await?;

	let req_list_payments = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_payments"
		}),
	);
	req_list_payments.await?.print().await?;

	let req_logoff = hc.do_post(
		"/api/logoff",
		json!({
			"logoff": true
		}),
	);
	// req_logoff.await?.print().await?;

	Ok(())
}
