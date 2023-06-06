use crate::ctx::Ctx;
use crate::web::{self, ReqStamp};

use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use tracing::{debug, error};

pub async fn mw_response_map(
    ctx: Option<Ctx>,
    http_method: Method,
    uri: Uri,
    req_stamp: ReqStamp,
    res: Response,
) -> Response {
    debug!("{:<12} - main_response_mapper", "RES_MAPPER");

    let rpc_info = res.extensions().get::<RpcInfo>();

    // -- Get the eventual response error
    let web_error = res.extensions().get::<web::Error>();
    let client_status_error = web_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new response
    // let error_response = 
    // error_response.unwrap_or(res)
    todo!();
}
