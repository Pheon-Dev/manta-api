// region: --- Modules
mod config;
mod crypt;
mod ctx;
mod error;
mod model;
mod utils;
mod web;

// pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;

use crate::model::ModelManager;
use axum::{middleware, Router};
use tracing_subscriber::EnvFilter;
use std::net::SocketAddr;

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

    let all_routes = Router::new()
        .merge(web::login_routes::routes(mm.clone()))
        .fallback_service(web::static_routes::serve_dir());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("{:<12} - on {addr}\n", "LISTENING");
    axum::Server::bind(&addr)
        .serve(all_routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}
