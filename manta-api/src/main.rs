use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use serde_json::json;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

use manta_api_logic::{
    Pagination, SendRequest, SendRequestStore, SendRequestStoreError, UpdateSendRequest,
};

// use tower_cookies::CookieManagerLayer;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// use self::error::Result;
//
// mod error;

type Db = Arc<RwLock<SendRequestStore>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "manta_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = Db::default();

    let all_routes = Router::new()
        .route(
            "/send-requests",
            get(get_send_requests).post(add_send_request),
        )
        .route(
            "/send-requests/:id",
            delete(delete_send_request)
                .patch(update_send_request)
                .get(get_send_request),
        )
        .route("/send-requests/persist", post(persist))
        // .layer(CookieManagerLayer::new())
        .with_state(db)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );
    // .fallback(get(|| async {"hello"}));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    info!("{:<12} - on {addr}\n", "LISTENING");

    axum::Server::bind(&addr)
        .serve(all_routes.into_make_service())
        .await
        .unwrap();
}

async fn get_send_requests(
    pagination: Option<Query<Pagination>>,
    State(db): State<Db>,
) -> impl IntoResponse {
    let requests = db.read().await;
    let Query(pagination) = pagination.unwrap_or_default();
    Json(requests.get_send_requests(pagination))
}

async fn get_send_request(State(db): State<Db>, Path(id): Path<usize>) -> impl IntoResponse {
    let requests = db.read().await;
    if let Some(request) = requests.get_send_request(id) {
        Json(request).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Not found").into_response()
    }
}

async fn add_send_request(
    State(db): State<Db>,
    Json(request): Json<SendRequest>,
) -> impl IntoResponse {
    let mut requests = db.write().await;
    let request = requests.add_send_request(request);
    (StatusCode::CREATED, Json(request))
}

async fn delete_send_request(State(db): State<Db>, Path(id): Path<usize>) -> impl IntoResponse {
    if db.write().await.remove_send_request(id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn update_send_request(
    Path(id): Path<usize>,
    State(db): State<Db>,
    Json(request): Json<UpdateSendRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut requests = db.write().await;
    let res = requests.update_send_request(&id, request);
    match res {
        Some(request) => Ok(Json(request.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

enum AppError {
    UserRepo(SendRequestStoreError),
}

impl From<SendRequestStoreError> for AppError {
    fn from(err: SendRequestStoreError) -> Self {
        Self::UserRepo(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::UserRepo(SendRequestStoreError::FileAccessError(_)) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "File access error")
            }
            AppError::UserRepo(SendRequestStoreError::SerializationError(_)) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Serialization error")
            }
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

async fn persist(State(db): State<Db>) -> Result<(), AppError> {
    tracing::debug!("Persisting...");
    let requests = db.read().await;
    requests.persist().await?;
    Ok(())
}
