use crate::{error::Error, state::SharedAppState};
use axum::{extract::Path, extract::State, http::StatusCode, Json};
use forge_api_db::entities;
use tracing::info;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn create(
    State(app_state): State<SharedAppState>,
    Json(note): Json<entities::notes::NoteChangeset>,
) -> Result<(StatusCode, Json<entities::notes::Note>), Error> {
    info!("respondingggggggggggggggggggggg");
    let note = entities::notes::create(note, &app_state.db_pool).await?;
    info!("responding with {:?}", note);
    Ok((StatusCode::CREATED, Json(note)))
}

#[axum::debug_handler]
pub async fn read_all(
    State(app_state): State<SharedAppState>,
) -> Result<Json<Vec<entities::notes::Note>>, Error> {
    // /* Example:
    let notes = entities::notes::load_all(&app_state.db_pool).await?;

    info!("responding with {:?}", notes);

    Ok(Json(notes))
}

#[axum::debug_handler]
pub async fn read_one(
    State(app_state): State<SharedAppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<entities::notes::Note>, Error> {
    let note = entities::notes::load(id, &app_state.db_pool).await?;
    Ok(Json(note))
}

#[axum::debug_handler]
pub async fn update(
    State(app_state): State<SharedAppState>,
    Path(id): Path<Uuid>,
    Json(note): Json<entities::notes::NoteChangeset>,
) -> Result<Json<entities::notes::Note>, Error> {
    let note = entities::notes::update(id, note, &app_state.db_pool).await?;
    Ok(Json(note))
}

#[axum::debug_handler]
pub async fn delete(
    State(app_state): State<SharedAppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, Error> {
    entities::notes::delete(id, &app_state.db_pool).await?;
    Ok(StatusCode::NO_CONTENT)
}
