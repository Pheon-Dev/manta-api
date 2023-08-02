use crate::ctx::Ctx;
use crate::model::card::{Card, CardBmc, CardForCreate, CardForUpdate};
use crate::model::ModelManager;
use crate::web::rpc::{DataResult, ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::web::Result;

pub async fn create_card(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<CardForCreate>,
) -> Result<DataResult<Card>> {
	let ParamsForCreate { data } = params;

	let id = CardBmc::create(&ctx, &mm, data).await?;
	let card = CardBmc::get(&ctx, &mm, id).await?;

	Ok(DataResult::new(card))
}

pub async fn get_card(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<DataResult<Card>> {
	let ParamsIded { id } = params;

	let card = CardBmc::get(&ctx, &mm, id).await?;
	CardBmc::get(&ctx, &mm, id).await?;

	Ok(DataResult::new(card))
}

pub async fn list_cards(
	ctx: Ctx,
	mm: ModelManager,
) -> Result<DataResult<Vec<Card>>> {
	let cards = CardBmc::list(&ctx, &mm).await?;

	Ok(DataResult::new(cards))
}

pub async fn update_card(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<CardForUpdate>,
) -> Result<DataResult<Card>> {
	let ParamsForUpdate { id, data } = params;

	CardBmc::update(&ctx, &mm, id, data).await?;

	let card = CardBmc::get(&ctx, &mm, id).await?;

	Ok(DataResult::new(card))
}

pub async fn delete_card(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<DataResult<Card>> {
	let ParamsIded { id } = params;

	let card = CardBmc::get(&ctx, &mm, id).await?;
	CardBmc::delete(&ctx, &mm, id).await?;

	Ok(DataResult::new(card))
}
