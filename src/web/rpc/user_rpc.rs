use crate::ctx::Ctx;
use crate::model::user::{User, UserBmc, UserForCreate, UserForUpdate};
use crate::model::ModelManager;
use crate::web::rpc::{DataResult, ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::web::Result;

pub async fn create_user(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<UserForCreate>,
) -> Result<DataResult<User>> {
	let ParamsForCreate { data } = params;
	let id = UserBmc::create(&ctx, &mm, data).await?;
	let user = UserBmc::get(&ctx, &mm, id).await?;
	Ok(DataResult::new(user))
}

pub async fn list_users(
	ctx: Ctx,
	mm: ModelManager,
) -> Result<DataResult<Vec<User>>> {
	let users = UserBmc::list(&ctx, &mm).await?;

	Ok(DataResult::new(users))
}

pub async fn update_user(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<UserForUpdate>,
) -> Result<DataResult<User>> {
	let ParamsForUpdate { id, data } = params;

	UserBmc::update(&ctx, &mm, id, data).await?;

	let user = UserBmc::get(&ctx, &mm, id).await?;

	Ok(DataResult::new(user))
}

pub async fn delete_user(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<DataResult<User>> {
	let ParamsIded { id } = params;

	let user = UserBmc::get(&ctx, &mm, id).await?;
	UserBmc::delete(&ctx, &mm, id).await?;

	Ok(DataResult::new(user))
}

// pub async fn get_user(
// 	ctx: Ctx,
// 	mm: ModelManager,
// 	params: ParamsIded,
// ) -> Result<DataResult<User>> {
// 	let ParamsIded { id } = params;
//
// 	let user = UserBmc::get(&ctx, &mm, id).await?;
// 	UserBmc::get_user(&ctx, &mm, id).await?;
//
// 	Ok(DataResult::new(user))
// }
