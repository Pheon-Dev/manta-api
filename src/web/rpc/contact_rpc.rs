use crate::ctx::Ctx;
use crate::model::contact::{
	Contact, ContactBmc, ContactForCreate, ContactForUpdate,
};
use crate::model::ModelManager;
use crate::web::rpc::{DataResult, ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::web::Result;

pub async fn create_contact(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<ContactForCreate>,
) -> Result<DataResult<Contact>> {
	let ParamsForCreate { data } = params;

	let id = ContactBmc::create(&ctx, &mm, data).await?;
	let contact = ContactBmc::get(&ctx, &mm, id).await?;

	Ok(DataResult::new(contact))
}

pub async fn list_contacts(
	ctx: Ctx,
	mm: ModelManager,
) -> Result<DataResult<Vec<Contact>>> {
	let contacts = ContactBmc::list(&ctx, &mm).await?;

	Ok(DataResult::new(contacts))
}

pub async fn update_contact(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<ContactForUpdate>,
) -> Result<DataResult<Contact>> {
	let ParamsForUpdate { id, data } = params;

	ContactBmc::update(&ctx, &mm, id, data).await?;

	let contact = ContactBmc::get(&ctx, &mm, id).await?;

	Ok(DataResult::new(contact))
}

pub async fn delete_contact(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<DataResult<Contact>> {
	let ParamsIded { id } = params;

	let contact = ContactBmc::get(&ctx, &mm, id).await?;
	ContactBmc::delete(&ctx, &mm, id).await?;

	Ok(DataResult::new(contact))
}
