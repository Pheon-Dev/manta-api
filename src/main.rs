// region: --- Modules
mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod utils;
mod web;

// pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;
use tower_cookies::CookieManagerLayer;

use crate::model::ModelManager;
use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_req_stamp::mw_req_stamp;
use crate::web::{login_routes, static_routes};

use axum::{middleware, Router};
use tracing_subscriber::EnvFilter;
use std::net::SocketAddr;
use tracing::info;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY 
    // _dev_utils::init_dev().await;

    let mm = ModelManager::new().await?;

    // TODO: routes rpc
    // let routes_rpc = 

    let all_routes = Router::new()
        .merge(web::login_routes::routes(mm.clone()))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(middleware::from_fn(mw_req_stamp))
        .layer(CookieManagerLayer::new())
        .fallback_service(web::static_routes::serve_dir());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("{:<12} - on {addr}\n", "LISTENING");
    axum::Server::bind(&addr)
        .serve(all_routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}
