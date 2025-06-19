use crate::controllers::notes;
use crate::middlewares::auth::auth;
use crate::state::AppState;
use axum::{
    http::{header, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};

use crate::auth::{login_handler, registeration_handler};
use forge_api_db::entities::users::User;
use jwt_lib::AuthUser;
use serde_json::json;
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
        .route("/login", post(login_handler))
        .route("/register", post(registeration_handler))
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
