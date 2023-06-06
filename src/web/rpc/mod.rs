// region: --- Modules
mod send_req_rpc;

use crate::web::rpc::send_req_rpc::{create_send_req, list_send_reqs, update_send_req, delete_send_req};
use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::web::{Result, Error};

use axum::{Json, Router};
use axum::routing::post;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use serde_json::{Value, json, from_value, to_value};
use tracing::debug;

// endregion: --- Modules

/// RPC Request Body coming from the client RPC Call.
#[derive(Deserialize)]
struct RpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

/// RPC basic information holding the id and method for further logging
#[derive(Debug)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

// region: --- Params Types
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

// endregion: --- Params Types

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
    .route("/rpc", post(rpc_handler))
        .with_state(mm)
}

macro_rules! exec_rpc_fn {
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

    // without params
    ($rpc_fn:expr, $ctx:expr, $mm:expr) => {{
        $rpc_fn($ctx, $mm).await.map(to_value)??
    }};
}

async fn rpc_handler(
    State(mm): State<ModelManager>,
ctx: Ctx,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    // -- Create the RPC Context to be set to the response.extensions
    let rpc_info = RpcInfo {
    id: rpc_req.id.clone(),
    method: rpc_req.method.clone(),
};

    // -- Exec & Store RPC Info in response.
    let mut res = rpc_inner_handler(ctx, mm, rpc_req).await.into_response();
    res.extensions_mut().insert(rpc_info);
    res
}

async fn rpc_inner_handler(
    ctx: Ctx,
    mm: ModelManager,
    rpc_req: RpcRequest,
) -> Result<Json<Value>> {
    let RpcRequest {
    id: rpc_id,
    method: rpc_method,
    params: rpc_params
} = rpc_req;

    debug!("{:<12} - rpc_handler_inner - method: {rpc_method}", "HANDLER");
    let result_body = match rpc_method.as_str() {
        // Send RPC methods
        "create_send_req" => exec_rpc_fn!(create_send_req, ctx, mm, rpc_params),
        "list_send_reqs" => exec_rpc_fn!(list_send_reqs, mm,ctx),
        "update_send_req" => exec_rpc_fn!(update_send_req, ctx, mm, rpc_params),
        "delete_send_req" => exec_rpc_fn!(delete_send_req, ctx, mm, rpc_params),

        // -- Fallback as Err
        _ => return Err(Error::RpcMethodUnknown(rpc_method)),
    };

    let body_response = json!({
        "id": rpc_id,
        "result": result_body,
    });

    Ok(Json(body_response))
}
