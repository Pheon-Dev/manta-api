use crate::utils::now_utc;
use crate::web::{Error, ReqStamp, Result};

use axum::middleware::Next;
use axum::http::Request;
use axum::response::Response;
use tracing::debug;
use uuid::Uuid;

pub async fn mw_req_stamp<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    debug!("{:<12} - mw_req_stamp_resolver", "MIDDLEWARE");

    let time_in = now_utc();
    let uuid = Uuid::new_v4();

    req.extensions_mut().insert(ReqStamp { uuid, time_in });

    Ok(next.run(req).await)
}

// TODO: region: --- ReqStamp Extractor
