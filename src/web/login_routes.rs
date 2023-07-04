use crate::{web, Error, Result};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use utoipa::ToSchema;

pub fn routes() -> Router {
	Router::new().route("/api/login", post(login_api))
}

#[utoipa::path(
    post,
    path = "/api/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Login Success", body = LoginResponse),
        (status = 404, description = "Login Fail"),
    )
)]
async fn login_api(
	cookies: Cookies,
	payload: Json<LoginPayload>,
) -> Result<Json<Value>> {
	println!("->> {:<12} - login_api", "HANDLER");

	// TODO: Implement real db/auth logic.
	if payload.username != "demo1" || payload.pwd != "welcome" {
		return Err(Error::LoginFail);
	}

	// FIXME: Implement real auth-token generation/signature.
	cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

	// Create the success body.
	let body = Json(json!({
		"action": "login",
        "success": true,
	}));

	Ok(body)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginPayload {
	username: String,
	pwd: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginResponse {
	pub action: String,
	pub success: bool,
}
