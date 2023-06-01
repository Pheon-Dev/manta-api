use mod config;

pub use self::error::{Error, Result};
use axum::Router;
use axum::extract::Query;
use axum::response::{IntoResponse, Html};
use axum::routing::get;
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<()> {
    let all_routes = Router.merge(routes_hello());
    Ok(())
}
