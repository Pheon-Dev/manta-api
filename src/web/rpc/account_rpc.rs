use crate::ctx::Ctx;
use crate::model::account::{
	Account, AccountBmc, AccountForCreate, AccountForUpdate,
};
use crate::model::ModelManager;
use crate::web::rpc::{DataResult, ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::web::Result;

pub async fn create_account(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<AccountForCreate>,
) -> Result<DataResult<Account>> {
	let ParamsForCreate { data } = params;

	let id = AccountBmc::create(&ctx, &mm, data).await?;
	let account = AccountBmc::get(&ctx, &mm, id).await?;

	Ok(DataResult::new(account))
}

pub async fn list_accounts(
	ctx: Ctx,
	mm: ModelManager,
) -> Result<DataResult<Vec<Account>>> {
	let accounts = AccountBmc::list(&ctx, &mm).await?;

	Ok(DataResult::new(accounts))
}

pub async fn update_account(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<AccountForUpdate>,
) -> Result<DataResult<Account>> {
	let ParamsForUpdate { id, data } = params;

	AccountBmc::update(&ctx, &mm, id, data).await?;

	let account = AccountBmc::get(&ctx, &mm, id).await?;

	Ok(DataResult::new(account))
}

pub async fn delete_account(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<DataResult<Account>> {
	let ParamsIded { id } = params;

	let account = AccountBmc::get(&ctx, &mm, id).await?;
	AccountBmc::delete(&ctx, &mm, id).await?;

	Ok(DataResult::new(account))
}
