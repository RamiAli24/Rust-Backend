use crate::state::SharedAppState;

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{json, Value};
use tracing::info;

use bcrypt::{hash, verify, DEFAULT_COST};
use forge_api_db::entities::users::{find_user_by_name, insert_user};

#[axum::debug_handler]
pub async fn login_handler(
    State(app_state): State<SharedAppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let name = payload.get("name").and_then(|v| v.as_str());
    let pass = payload.get("password").and_then(|v| v.as_str());

    if name.is_none() || pass.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"success": false, "message": "Missing name or password"})),
        );
    }

    // Extract the actual &str from Option<&str>
    let name = name.unwrap(); // Safe because we checked above

    // This requires your function to return Result<impl IntoResponse, (StatusCode, Json<Value>)>
    let user = match find_user_by_name(name, &app_state.db_pool).await {
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"success": false, "message": "Database error"})),
            )
        }
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"success": false, "message": "Invalid credentials"})),
            )
        }
        Ok(Some(user)) => user,
    };
    // Validate password
    let pass = pass.unwrap(); // Safe because we checked above

    let password_ok = verify(&pass, &user.pass).unwrap_or(false);
    info!("password_ok {}", password_ok);

    println!("Stored hash: '{}'", &user.pass);
    if !password_ok {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"success": false, "message": "Invalid credentials"})),
        );
    }

    match jwt_lib::get_jwt(user).await {
        Ok(token) => (
            StatusCode::OK,
            Json(json!({"success": true, "data": { "token": token}})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success": false, "message": e})),
        ),
    }
}

#[axum::debug_handler]
pub async fn registeration_handler(
    State(app_state): State<SharedAppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let name = payload.get("name").and_then(|v| v.as_str());
    let pass = payload.get("password").and_then(|v| v.as_str());

    if name.is_none() || pass.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"success": false, "message": "Missing name or password"})),
        );
    }
    // shadow variable
    let pass = pass.unwrap();
    let hashed_pass = hash(&pass, DEFAULT_COST);
    let hashed_pass = hashed_pass.unwrap();
    // Extract the actual &str from Option<&str>
    let name = name.unwrap(); // Safe because we checked above

    match insert_user(name, &hashed_pass, &app_state.db_pool).await {
        Err(err) => {
            eprintln!("Database insert error: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"success": false, "message": "Database error"})),
            )
        }

        Ok(user) => (
            StatusCode::OK,
            Json(json!({"success": true, "data": { "user": user.name}})),
        ),
    }
}
