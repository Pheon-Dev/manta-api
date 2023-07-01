use std::net::SocketAddr;

use manta_api_logic::{SendRequest, UpdateSendRequest};

use axum::Router;
use axum::routing::get;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use self::error::Result;

mod error;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "manta_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let all_routes = Router::new()
        .route("/", get(|| async { "Hello, World!"}))
        .layer(CookieManagerLayer::new())
        .fallback(get(|| async {"hello"}));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    info!("{:<12} - on {addr}\n", "LISTENING");

    axum::Server::bind(&addr)
        .serve(all_routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}
