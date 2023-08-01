use crate::ctx::Ctx;
use crate::model::contact::{Contact, ContactForCreate, ModelController};
use crate::Result;

use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};

pub fn routes(omc: ModelController) -> Router {
	Router::new()
		.route("/contacts", post(create_contact).get(list_contacts))
		.route("/contacts/delete/:id", delete(delete_contact))
		.route("/contacts/details/:id", get(details_contact))
		.with_state(omc)
}

// region:    --- REST Handlers
#[utoipa::path(
    post,
    path = "/api/contacts",
    request_body = ContactForCreate,
    responses((
        status = 200,
        body = [Contact],
        description = "Contact",
    ), ( status = 404, description = "Contact Not Found"))
)]
async fn create_contact(
	State(omc): State<ModelController>,
	ctx: Ctx,
	Json(contact_fc): Json<ContactForCreate>,
) -> Result<Json<Contact>> {
	println!("->> {:<12} - create_contact", "HANDLER");

	let contact = omc.create_contact(ctx, contact_fc).await?;

	Ok(Json(contact))
}

#[utoipa::path(
    get,
    path = "/api/contacts",
    responses((
        status = 200,
        body = [Contact],
        description = "Contacts List",
    ), ( status = 404, description = "Contact List Not Found"))
)]
async fn list_contacts(
	State(omc): State<ModelController>,
	ctx: Ctx,
) -> Result<Json<Vec<Contact>>> {
	println!("->> {:<12} - list_contacts", "HANDLER");

	let contacts = omc.list_contacts(ctx).await?;

	Ok(Json(contacts))
}

#[utoipa::path(
    get,
    path = "/api/contacts/details/{id}",
    params(("id" = u64, Path, description = "Contact ID")),
    responses((
        status = 200,
        body = Contact,
        description = "Contact Details",
    ), ( status = 404, description = "Contact Details Not Found"))
)]
async fn details_contact(
	State(omc): State<ModelController>,
	ctx: Ctx,
	Path(id): Path<u64>,
) -> Result<Json<Contact>> {
	println!("->> {:<12} - contact_details", "HANDLER");

	let contact = omc.details_contact(ctx, id).await?;

	Ok(Json(contact))
}

#[utoipa::path(
    delete,
    path = "/api/contacts/delete/{id}",
    params(("id" = u64, Path, description = "Contact ID")),
    responses((
        status = 200,
        description = "Contact Deleted",
    ), (status = 404, description = "Failed to Delete Contact"))
)]
async fn delete_contact(
	State(omc): State<ModelController>,
	ctx: Ctx,
	Path(id): Path<u64>,
) -> Result<Json<Contact>> {
	println!(">>> {:<12} - delete_contact", "HANDLER");

	let contact = omc.delete_contact(ctx, id).await?;

	Ok(Json(contact))
}
// endregion: --- REST Handlers
