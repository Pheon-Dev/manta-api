use crate::crypt::pwd::{self, SchemeStatus};
use crate::crypt::EncryptContent;
use crate::ctx::Ctx;
use crate::model::user::{UserBmc, UserForLogin};
use crate::model::ModelManager;
use crate::web::{self, remove_token_cookie};
use crate::web::{Error, Result};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::debug;
use utoipa::ToSchema;

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/api/login", post(login_api_handler))
		.route("/api/logoff", post(logoff_api_handler))
		.with_state(mm)
}

// region:    --- Login
#[utoipa::path(
   post, 
    path = "/api/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Login Success"),
        (status = 404, description = "Login Fail"),
    )
)]
async fn login_api_handler(
	State(mm): State<ModelManager>,
	cookies: Cookies,
	Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
	debug!("{:<12} - api_login_handler", "HANDLER");

	let LoginPayload { username, pwd: pwd_clear } = payload;
	let root_ctx = Ctx::root_ctx();

	// -- Get the user.
	let user: UserForLogin = UserBmc::first_by_username(&root_ctx, &mm, &username)
		.await?
		.ok_or(Error::LoginFailUsernameNotFound)?;
	let user_id = user.id;

	// -- Validate the password.
	let Some(pwd) = user.pwd else {
		return Err(Error::LoginFailUserHasNoPwd{ user_id }) ;
	};

	let scheme_status = pwd::validate_pwd(
		&EncryptContent {
			salt: user.pwd_salt.to_string(),
			content: pwd_clear.clone(),
		},
		&pwd,
	)
	.map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

	// -- If pwd scheme outdated, update pwd.
	if let SchemeStatus::Outdated = scheme_status {
		debug!("pwd encrypt scheme outdated, upgrading.");
		UserBmc::update_pwd(&root_ctx, &mm, user.id, &pwd_clear).await?;
	}

	// -- Set web token.
	web::set_token_cookie(&cookies, &user.username, &user.token_salt.to_string())?;

	// -- Create the success body.
	let body = Json(json!({
		"result": {
			"success": true
		}
	}));

	Ok(body)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginPayload {
	username: String,
	pwd: String,
}
// endregion: --- Login

// region:    --- Logoff
#[utoipa::path(
   post, 
    path = "/api/logoff",
    request_body = LogoffPayload,
    responses(
        (status = 200, description = "Logoff Success"),
        (status = 404, description = "Logoff Fail"),
    )
)]
async fn logoff_api_handler(
	cookies: Cookies,
	Json(payload): Json<LogoffPayload>,
) -> Result<Json<Value>> {
	debug!("{:<12} - api_logoff_handler", "HANDLER");
	let should_logoff = payload.logoff;

	if should_logoff {
		remove_token_cookie(&cookies)?;
	}

	// Create the success body.
	let body = Json(json!({
		"result": {
			"logged_off": should_logoff
		}
	}));

	Ok(body)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LogoffPayload {
	logoff: bool,
}
// endregion: --- Logoff
