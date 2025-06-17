use crate::controllers::notes;
use crate::middlewares::auth::auth;
use crate::state::AppState;
use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

/// Initializes the application's routes.
///
/// This function maps paths (e.g. "/greet") and HTTP methods (e.g. "GET") to functions in [`crate::controllers`] as well as includes middlewares defined in [`crate::middlewares`] into the routing layer (see [`axum::Router`]).
pub fn init_routes(app_state: AppState) -> Router {
    let shared_app_state = Arc::new(app_state);
    Router::new()
        .route("/notes/{id}", delete(notes::delete))
        .route("/notes/{id}", put(notes::update))
        .route_layer(middleware::from_fn_with_state(
            shared_app_state.clone(),
            auth,
        ))
        .route("/notes", get(notes::read_all))
        .route("/notes", post(notes::create))
        .route("/notes/{id}", get(notes::read_one))
        .fallback(fallback_handler)
        .with_state(shared_app_state)
}

#[axum::debug_handler]
pub async fn fallback_handler() -> impl IntoResponse {
    let mut body = HashMap::new();
    body.insert("error".to_string(), "not found".to_string());
    info!("Fallback handler triggered: route not found");
    (StatusCode::NOT_FOUND, Json(body))
}
