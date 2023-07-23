// #![allow(unused)] // For early development stages.

// region:    --- Modules

mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod utils;
mod web;

// #[cfg(test)] // Commented during early development.
pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;

use crate::model::payment::{Payment, PaymentForCreate, PaymentForUpdate};
use crate::model::ModelManager;
use crate::web::login_routes::{LoginPayload, LogoffPayload};
use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_req_stamp::mw_req_stamp;
use crate::web::mw_res_map::mw_response_map;
use crate::web::{login_routes, rpc, static_routes};

use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// endregion: --- Modules

#[derive(OpenApi)]
#[openapi(
    paths(
        web::login_routes::login_api_handler,
        web::login_routes::logoff_api_handler,
        web::rpc::payment_rpc::create_payment,
        web::rpc::payment_rpc::list_payments,
        web::rpc::payment_rpc::update_payment,
        web::rpc::payment_rpc::delete_payment,
    ),
    components(
        schemas(LoginPayload, LogoffPayload, Payment, PaymentForCreate, PaymentForUpdate)
    ),
    tags((name = "Manta API", description = "Money Transfer API"))
)]

struct ApiDoc;

#[tokio::main]
async fn main() -> Result<()> {
	let cors = CorsLayer::new().allow_origin(Any);

	tracing_subscriber::fmt()
		.without_time() // For early local development.
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	// -- FOR DEV ONLY
	_dev_utils::init_dev().await;

	// -- Initialize ModelController
	let mm = ModelManager::new().await?;

	// -- Define Routes
	let rpc_routes =
		rpc::routes(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));

	let all_routes = Router::new()
		.merge(login_routes::routes(mm.clone()))
		.nest("/api", rpc_routes)
		.merge(
			SwaggerUi::new("/manta-ui")
				.url("/api-doc/apenapi.json", ApiDoc::openapi()),
		)
		.layer(middleware::map_response(mw_response_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		.layer(middleware::from_fn(mw_req_stamp))
		.layer(CookieManagerLayer::new())
		.layer(cors)
		.fallback_service(static_routes::serve_dir());

	// region:    --- Start Server
	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	info!("{:<12} - on {addr}\n", "LISTENING");
	axum::Server::bind(&addr)
		.serve(all_routes.into_make_service())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}
