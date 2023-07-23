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
			"username": "demo1",
			"password": "welcome"
		}),
	);
	req_login.await?.print().await?;

	let req_create_send = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_send",
			"params": {
				"data": {
					"amount": "100"
				}
			}
		}),
	);
	req_create_send.await?.print().await?;

	let req_update_send = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_send",
			"params": {
				"id": 1000, // Hardcode the send id.
				"data": {
					"amount": "200"
				}
			}
		}),
	);
	// req_update_send.await?.print().await?;

	let req_delete_send = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "delete_send",
			"params": {
				"id": 1001 // Harcode the send id
			}
		}),
	);
	// req_delete_send.await?.print().await?;

	let req_list_sends = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_sends"
		}),
	);
	req_list_sends.await?.print().await?;

	let req_logoff = hc.do_post(
		"/api/logoff",
		json!({
			"logoff": true
		}),
	);
	// req_logoff.await?.print().await?;

	Ok(())
}
