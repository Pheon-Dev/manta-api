// #![allow(unused)] // For beginning only.

pub use self::error::{Error, Result};

use crate::ctx::Ctx;
use crate::log::log_request;
use crate::model::{ModelController, Payment, PaymentForCreate};
use crate::web::login_routes::{LoginPayload, LoginResponse};

use axum::handler::HandlerWithoutStateExt;
use axum::http::{Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::{any_service, MethodRouter};
use axum::{middleware, Json, Router};
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[derive(OpenApi)]
#[openapi(
    paths(
        web::login_routes::login_api,
        web::payments_routes::create_payment,
        web::payments_routes::list_payments,
        web::payments_routes::details_payment,
        web::payments_routes::delete_payment,
    ),
    components(
        schemas(LoginPayload, LoginResponse, Payment, PaymentForCreate),
    ),
    tags((name = "Manta API", description = "A payments web application API")),
)]

struct ApiDoc;
#[tokio::main]
async fn main() -> Result<()> {
	let cors = CorsLayer::new().allow_origin(Any);
	// Initialize ModelController.
	let mc = ModelController::new().await?;

	// println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
	let routes_apis = web::payments_routes::routes(mc.clone())
		.route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

	let routes_all = Router::new()
		.merge(
			SwaggerUi::new("/manta-ui")
				.url("/api-doc/openapi.json", ApiDoc::openapi()),
		)
		.merge(web::login_routes::routes())
		.nest("/api", routes_apis)
		.layer(middleware::map_response(main_response_mapper))
		.layer(middleware::from_fn_with_state(
			mc.clone(),
			web::mw_auth::mw_ctx_resolver,
		))
		.layer(CookieManagerLayer::new())
		.layer(cors)
		.fallback_service(routes_static());

	// region:    --- Start Server
	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	println!("->> LISTENING on {addr}\n");
	axum::Server::bind(&addr)
		.serve(routes_all.into_make_service())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}

async fn main_response_mapper(
	ctx: Option<Ctx>,
	uri: Uri,
	req_method: Method,
	res: Response,
) -> Response {
	println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
	let uuid = Uuid::new_v4();

	// -- Get the eventual response error.
	let service_error = res.extensions().get::<Error>();
	let client_status_error = service_error.map(|se| se.client_status_and_error());

	// -- If client error, build the new reponse.
	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error_body = json!({
					"error": {
						"type": client_error.as_ref(),
						"req_uuid": uuid.to_string(),
					}
				});

				println!("    ->> client_error_body: {client_error_body}");

				// Build the new response from the client_error_body
				(*status_code, Json(client_error_body)).into_response()
			});

	// Build and log the server log line.
	let client_error = client_status_error.unzip().1;
	// TODO: Need to hander if log_request fail (but should not fail request)
	let _ =
		log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

	println!();
	error_response.unwrap_or(res)
}

const WEB_FOLDER: &str = "wallet";

fn routes_static() -> MethodRouter {
	async fn handle_404() -> (StatusCode, &'static str) {
		(StatusCode::NOT_FOUND, "404 Page Not Found")
	}

	any_service(
		ServeDir::new(WEB_FOLDER).not_found_service(handle_404.into_service()),
	)
}
