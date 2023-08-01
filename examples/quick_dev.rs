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

	let req_create_user = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_user",
			"params": {
				"data": {
					"username": "johnpaull",
					"password_clear": "welcome",
				}
			}
		}),
	);

	let req_update_user = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_user",
			"params": {
				"id": 1001, // Hardcode the send id.
				"data": {
					"username": "johnpaul"
				}
			}
		}),
	);

	let req_delete_user = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "delete_user",
			"params": {
				"id": 1001 // Harcode the send id
			}
		}),
	);

	let req_list_users = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_users"
		}),
	);

	req_create_user.await?.print().await?;
	// req_update_user.await?.print().await?;
	// // req_delete_user.await?.print().await?;
	req_list_users.await?.print().await?;

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

	let req_update_payment = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_payment",
			"params": {
				"id": 1003, // Hardcode the send id.
				"data": {
					"amount": "200"
				}
			}
		}),
	);

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

	let req_list_payments = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_payments"
		}),
	);

	// req_create_payment.await?.print().await?;
	// req_update_payment.await?.print().await?;
	// // req_delete_payment.await?.print().await?;
	// req_list_payments.await?.print().await?;

	let req_create_contact = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_contact",
			"params": {
				"data": {
					"ref_id": "7jk4h37g",
					"association": "Business",
					"username": "soko",
					"name": "Soko Brand",
					"email": "soko@brand.com",
				}
			}
		}),
	);

	let req_update_contact = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_contact",
			"params": {
				"id": 1000, // Hardcode the send id.
				"data": {
					"email": "soko@brand.net",
				}
			}
		}),
	);

	let req_delete_contact = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "delete_contact",
			"params": {
				"id": 1001 // Harcode the send id
			}
		}),
	);

	let req_list_contacts = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_contacts"
		}),
	);

	// req_create_contact.await?.print().await?;
	// req_update_contact.await?.print().await?;
	// // req_delete_contact.await?.print().await?;
	// req_list_contacts.await?.print().await?;

	let req_create_card = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_card",
			"params": {
				"data": {
					"cbalance": "10200",
					"cnumber": "4862-6345-6367-2456",
					"cvalid": "03/24",
					"cvv": "034",
					"cdescription": "M-PESA Global Pay",
					"cname": "M-PESA",
					"ctype": "VISA",
					"caccount": "Debit",
				}
			}
		}),
	);

	let req_update_card = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_card",
			"params": {
				"id": 1000, // Hardcode the send id.
				"data": {
					"cbalance": "20000"
				}
			}
		}),
	);

	let req_delete_card = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "delete_card",
			"params": {
				"id": 1001 // Harcode the send id
			}
		}),
	);

	let req_list_cards = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_cards"
		}),
	);

	// req_create_card.await?.print().await?;
	// req_update_card.await?.print().await?;
	// // req_delete_card.await?.print().await?;
	// req_list_cards.await?.print().await?;

	let req_create_account = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_account",
			"params": {
				"data": {
					"username": "janedoe",
					"email": "janedoe@email.com",
					"balance": "82000",
					"aid": "8j5ka89d",
					"cookie": "auth-token=amFuZWRvZQ.MjAyMy0wOC0wMVQxMDowMjoyNi4yODk0NDU2NzRa.5-dX3wPicTHhg_DpscgMXPBCrZA7Whd-OYrESugB6SyAA8QUzhna3DCviLt4c2BfL9Fy1PHzS4qEe2ZgVNHs0w",
				}
			}
		}),
	);

	let req_update_account = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_account",
			"params": {
				"id": 1000, // Hardcode the send id.
				"data": {
					"balance": "20000"
				}
			}
		}),
	);

	let req_delete_account = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "delete_account",
			"params": {
				"id": 1001 // Harcode the send id
			}
		}),
	);

	let req_list_accounts = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_accounts"
		}),
	);

	// req_create_account.await?.print().await?;
	// req_update_account.await?.print().await?;
	// // req_delete_account.await?.print().await?;
	// req_list_accounts.await?.print().await?;

	let req_logoff = hc.do_post(
		"/api/logoff",
		json!({
			"logoff": true
		}),
	);
	// req_logoff.await?.print().await?;

	Ok(())
}
