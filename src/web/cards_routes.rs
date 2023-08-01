use crate::ctx::Ctx;
use crate::model::card::{Card, CardForCreate, ModelController};
use crate::Result;

use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};

pub fn routes(cmc: ModelController) -> Router {
	Router::new()
		.route("/cards", post(create_card).get(list_cards))
		.route("/cards/delete/:id", delete(delete_card))
		.route("/cards/details/:id", get(details_card))
		.with_state(cmc)
}

// region:    --- REST Handlers
#[utoipa::path(
    post,
    path = "/api/cards",
    request_body = CardForCreate,
    responses((
        status = 200,
        body = [Card],
        description = "Card",
    ), ( status = 404, description = "Card Not Found"))
)]
async fn create_card(
	State(cmc): State<ModelController>,
	ctx: Ctx,
	Json(card_fc): Json<CardForCreate>,
) -> Result<Json<Card>> {
	println!("->> {:<12} - create_card", "HANDLER");

	let card = cmc.create_card(ctx, card_fc).await?;

	Ok(Json(card))
}

#[utoipa::path(
    get,
    path = "/api/cards",
    responses((
        status = 200,
        body = [Card],
        description = "Cards List",
    ), ( status = 404, description = "Card List Not Found"))
)]
async fn list_cards(
	State(cmc): State<ModelController>,
	ctx: Ctx,
) -> Result<Json<Vec<Card>>> {
	println!("->> {:<12} - list_cards", "HANDLER");

	let cards = cmc.list_cards(ctx).await?;

	Ok(Json(cards))
}

#[utoipa::path(
    get,
    path = "/api/cards/details/{id}",
    params(("id" = u64, Path, description = "Card ID")),
    responses((
        status = 200,
        body = Card,
        description = "Card Details",
    ), ( status = 404, description = "Card Details Not Found"))
)]
async fn details_card(
	State(cmc): State<ModelController>,
	ctx: Ctx,
	Path(id): Path<u64>,
) -> Result<Json<Card>> {
	println!("->> {:<12} - card_details", "HANDLER");

	let card = cmc.details_card(ctx, id).await?;

	Ok(Json(card))
}

#[utoipa::path(
    delete,
    path = "/api/cards/delete/{id}",
    params(("id" = u64, Path, description = "Card ID")),
    responses((
        status = 200,
        description = "Card Deleted",
    ), (status = 404, description = "Failed to Delete Card"))
)]
async fn delete_card(
	State(cmc): State<ModelController>,
	ctx: Ctx,
	Path(id): Path<u64>,
) -> Result<Json<Card>> {
	println!(">>> {:<12} - delete_card", "HANDLER");

	let card = cmc.delete_card(ctx, id).await?;

	Ok(Json(card))
}
// endregion: --- REST Handlers
