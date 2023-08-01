use crate::ctx::Ctx;
use crate::model::account::{Account, AccountForCreate, ModelController};
use crate::Result;

use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};

pub fn routes(amc: ModelController) -> Router {
	Router::new()
		.route("/accounts", post(create_account).get(list_accounts))
		.route("/accounts/delete/:id", delete(delete_account))
		.route("/accounts/details/:id", get(details_account))
		.with_state(amc)
}

// region:    --- REST Handlers
#[utoipa::path(
    post,
    path = "/api/accounts",
    request_body = AccountForCreate,
    responses((
        status = 200,
        body = [Account],
        description = "Account",
    ), ( status = 404, description = "Account Not Found"))
)]
async fn create_account(
	State(amc): State<ModelController>,
	ctx: Ctx,
	Json(account_fc): Json<AccountForCreate>,
) -> Result<Json<Account>> {
	println!("->> {:<12} - create_account", "HANDLER");

	let account = amc.create_account(ctx, account_fc).await?;

	Ok(Json(account))
}

#[utoipa::path(
    get,
    path = "/api/accounts",
    responses((
        status = 200,
        body = [Account],
        description = "Accounts List",
    ), ( status = 404, description = "Account List Not Found"))
)]
async fn list_accounts(
	State(amc): State<ModelController>,
	ctx: Ctx,
) -> Result<Json<Vec<Account>>> {
	println!("->> {:<12} - list_accounts", "HANDLER");

	let accounts = amc.list_accounts(ctx).await?;

	Ok(Json(accounts))
}

#[utoipa::path(
    get,
    path = "/api/accounts/details/{id}",
    params(("id" = u64, Path, description = "Account ID")),
    responses((
        status = 200,
        body = Account,
        description = "Account Details",
    ), ( status = 404, description = "Account Details Not Found"))
)]
async fn details_account(
	State(amc): State<ModelController>,
	ctx: Ctx,
	Path(id): Path<u64>,
) -> Result<Json<Account>> {
	println!("->> {:<12} - account_details", "HANDLER");

	let account = amc.details_account(ctx, id).await?;

	Ok(Json(account))
}

#[utoipa::path(
    delete,
    path = "/api/accounts/delete/{id}",
    params(("id" = u64, Path, description = "Account ID")),
    responses((
        status = 200,
        description = "Account Deleted",
    ), (status = 404, description = "Failed to Delete Account"))
)]
async fn delete_account(
	State(amc): State<ModelController>,
	ctx: Ctx,
	Path(id): Path<u64>,
) -> Result<Json<Account>> {
	println!(">>> {:<12} - delete_account", "HANDLER");

	let account = amc.delete_account(ctx, id).await?;

	Ok(Json(account))
}
// endregion: --- REST Handlers
