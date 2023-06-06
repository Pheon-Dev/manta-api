use crate::crypt::pwd::{self, SchemeStatus};
use crate::crypt::EncryptContent;
use crate::ctx::Ctx;
// use crate::model::user::{UserBmc, UserForLogin};
use crate::model::ModelManager;
use crate::web;
use crate::web::{Result, Error};

use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
    .route("/api/login", post(login_api_handler))
        .with_state(mm)
}

// region: --- Handler
async fn login_api_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_login_handler", "HANLDER");

    let LoginPayload { username, pwd: pwd_clear } = payload;
    let root_ctx = Ctx::root_ctx();

    // -- Get the user
    let user: UserForLogin
}
