use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use self::error::Result;

mod error;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
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
