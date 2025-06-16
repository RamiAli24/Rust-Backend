use crate::{error::Error, state::SharedAppState};
use axum::{extract::Path, extract::State, http::StatusCode, Json};
use forge_api_db::entities;
use tracing::info;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn create(
    State(app_state): State<SharedAppState>,
    Json(note): Json<() /* e.g.entities::notes::NoteChangeset */>,
) -> Result<() /* e.g. (StatusCode, Json<entities::notes::Note>) */, Error> {
    todo!("create resource via forge_api_db's APIs, trace, and respond!")

    /* Example:
    let note = entities::notes::create(note, &app_state.db_pool).await?;
    Ok((StatusCode::CREATED, Json(note)))
    */
}

#[axum::debug_handler]
pub async fn read_all(
    State(app_state): State<SharedAppState>,
) -> Result<() /* e.g. Json<Vec<entities::notes::Note>> */, Error> {
    todo!("load resources via forge_api_db's APIs, trace, and respond!")

    /* Example:
    let notes = entities::notes::load_all(&app_state.db_pool)
        .await?;

    info!("responding with {:?}", notes);

    Ok(Json(notes))
    */
}

#[axum::debug_handler]
pub async fn read_one(
    State(app_state): State<SharedAppState>,
    Path(id): Path<Uuid>,
) -> Result<() /* e.g. Json<entities::notes::Note> */, Error> {
    todo!("load resource via forge_api_db's APIs, trace, and respond!")

    /* Example:
    let note = entities::notes::load(id, &app_state.db_pool).await?;
    Ok(Json(note))
    */
}

#[axum::debug_handler]
pub async fn update(
    State(app_state): State<SharedAppState>,
    Path(id): Path<Uuid>,
    Json(note): Json<() /* e.g. entities::notes::NoteChangeset */>,
) -> Result<() /* e.g. Json<entities::notes::Note> */, Error> {
    todo!("update resource via forge_api_db's APIs, trace, and respond!")

    /* Example:
    let note = entities::notes::update(id, note, &app_state.db_pool).await?;
    Ok(Json(note))
    */
}

#[axum::debug_handler]
pub async fn delete(
    State(app_state): State<SharedAppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, Error> {
    todo!("delete resource via forge_api_db's APIs, trace, and respond!")

    /* Example:
    entities::notes::delete(id, &app_state.db_pool).await?;
    Ok(StatusCode::NO_CONTENT)
    */
}
