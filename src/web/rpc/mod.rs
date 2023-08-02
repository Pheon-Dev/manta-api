// region:    --- Modules

pub mod account_rpc;
pub mod card_rpc;
pub mod contact_rpc;
pub mod payment_rpc;
pub mod user_rpc;

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::web::rpc::account_rpc::{
	create_account, delete_account, get_account, list_accounts, update_account,
};
use crate::web::rpc::card_rpc::{
	create_card, delete_card, get_card, list_cards, update_card,
};
use crate::web::rpc::contact_rpc::{
	create_contact, delete_contact, get_contact, list_contacts, update_contact,
};
use crate::web::rpc::payment_rpc::{
	create_payment, delete_payment, get_payment, list_payments, update_payment,
};
use crate::web::rpc::user_rpc::{
	create_user,
	delete_user,
	// get_user,
	list_users,
	update_user,
};
use crate::web::{Error, Result};

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, to_value, Value};
use tracing::debug;

// endregion: --- Modules

// region:    --- RPC Types

/// JSON-RPC Request Body.
#[derive(Deserialize)]
struct RpcRequest {
	id: Option<Value>,
	method: String,
	params: Option<Value>,
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
	data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
	id: i64,
	data: D,
}

#[derive(Deserialize)]
pub struct ParamsIded {
	id: i64,
}

#[derive(Serialize)]
pub struct DataResult<D> {
	data: D,
}

impl<D> DataResult<D> {
	fn new(data: D) -> Self {
		Self { data }
	}
}
// endregion: --- RPC Types

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/rpc", post(rpc_handler))
		.with_state(mm)
}

async fn rpc_handler(
	State(mm): State<ModelManager>,
	ctx: Ctx,
	Json(rpc_req): Json<RpcRequest>,
) -> Response {
	// -- Create the RPC Info to be set to the response.extensions.
	let rpc_info = RpcInfo {
		id: rpc_req.id.clone(),
		method: rpc_req.method.clone(),
	};

	// -- Exec & Store RpcInfo in response.
	let mut res = _rpc_handler(ctx, mm, rpc_req).await.into_response();
	res.extensions_mut().insert(rpc_info);

	res
}

/// RPC basic information holding the id and method for further logging.
#[derive(Debug)]
pub struct RpcInfo {
	pub id: Option<Value>,
	pub method: String,
}

macro_rules! exec_rpc_fn {
	// With params.
	($rpc_fn:expr, $ctx:expr, $mm:expr, $rpc_params:expr) => {{
		let rpc_fn_name = stringify!($rpc_fn);
		let params = $rpc_params.ok_or(Error::RpcMissingParams {
			rpc_method: rpc_fn_name.to_string(),
		})?;
		let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
			rpc_method: rpc_fn_name.to_string(),
		})?;

		$rpc_fn($ctx, $mm, params).await.map(to_value)??
	}};

	// Without params.
	($rpc_fn:expr, $ctx:expr, $mm:expr) => {
		$rpc_fn($ctx, $mm).await.map(to_value)??
	};
}

async fn _rpc_handler(
	ctx: Ctx,
	mm: ModelManager,
	rpc_req: RpcRequest,
) -> Result<Json<Value>> {
	let RpcRequest {
		id: rpc_id,
		method: rpc_method,
		params: rpc_params,
	} = rpc_req;

	debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");

	let result_json = match rpc_method.as_str() {
		// -- payment RPC methods.
		"create_payment" => exec_rpc_fn!(create_payment, ctx, mm, rpc_params),
		"list_payments" => exec_rpc_fn!(list_payments, ctx, mm),
		"update_payment" => exec_rpc_fn!(update_payment, ctx, mm, rpc_params),
		"delete_payment" => exec_rpc_fn!(delete_payment, ctx, mm, rpc_params),
		"get_payment" => exec_rpc_fn!(get_payment, ctx, mm, rpc_params),
		"create_card" => exec_rpc_fn!(create_card, ctx, mm, rpc_params),
		"list_cards" => exec_rpc_fn!(list_cards, ctx, mm),
		"update_card" => exec_rpc_fn!(update_card, ctx, mm, rpc_params),
		"get_card" => exec_rpc_fn!(get_card, ctx, mm, rpc_params),
		"delete_card" => exec_rpc_fn!(delete_card, ctx, mm, rpc_params),
		"create_account" => exec_rpc_fn!(create_account, ctx, mm, rpc_params),
		"list_accounts" => exec_rpc_fn!(list_accounts, ctx, mm),
		"update_account" => exec_rpc_fn!(update_account, ctx, mm, rpc_params),
		"delete_account" => exec_rpc_fn!(delete_account, ctx, mm, rpc_params),
		"get_account" => exec_rpc_fn!(get_account, ctx, mm, rpc_params),
		"create_contact" => exec_rpc_fn!(create_contact, ctx, mm, rpc_params),
		"list_contacts" => exec_rpc_fn!(list_contacts, ctx, mm),
		"update_contact" => exec_rpc_fn!(update_contact, ctx, mm, rpc_params),
		"delete_contact" => exec_rpc_fn!(delete_contact, ctx, mm, rpc_params),
		"get_contact" => exec_rpc_fn!(get_contact, ctx, mm, rpc_params),
		"create_user" => exec_rpc_fn!(create_user, ctx, mm, rpc_params),
		"list_users" => exec_rpc_fn!(list_users, ctx, mm),
		"update_user" => exec_rpc_fn!(update_user, ctx, mm, rpc_params),
		"delete_user" => exec_rpc_fn!(delete_user, ctx, mm, rpc_params),
		// "get_user" => exec_rpc_fn!(get_user, ctx, mm, rpc_params),

		// -- Fallback as Err.
		_ => return Err(Error::RpcMethodUnknown(rpc_method)),
	};

	let body_response = json!({
		"id": rpc_id,
		"result": result_json
	});

	Ok(Json(body_response))
}
