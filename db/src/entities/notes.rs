#[cfg(feature = "test-helpers")]
use fake::{faker::lorem::en::*, Dummy};
use serde::Deserialize;
use serde::Serialize;
use sqlx::Postgres;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Debug, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub text: String,
}

#[derive(Deserialize, Validate, Clone)]
#[cfg_attr(feature = "test-helpers", derive(Serialize, Dummy))]
pub struct NoteChangeset {
    //#[cfg_attr(feature = "test-helpers", dummy(faker = "…()"))]
    //#[validate(…))]
    pub text: String,
}

pub async fn load_all(
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Vec<Note>, crate::Error> {
    let notes = sqlx::query_as!(Note, "SELECT id, text FROM notes")
        .fetch_all(executor)
        .await?;
    Ok(notes)
}

pub async fn load(
    id: Uuid,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Note, crate::Error> {
    match sqlx::query_as!(
        Note,
        "SELECT id, text FROM notes WHERE id = $1",
        id
    )
    .fetch_optional(executor)
    .await
    .map_err(crate::Error::DbError)?
    {
        Some(note) => Ok(note),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn create(
    note: NoteChangeset,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Note, crate::Error> {
    note.validate()?;

    let record = sqlx::query!(
        "INSERT INTO notes (text) VALUES ($1) RETURNING id",
        note.text,
    )
    .fetch_one(executor)
    .await
    .map_err(crate::Error::DbError)?;

    Ok(Note {
        id: record.id,
        text: note.text,
    })
}

pub async fn update(
    id: Uuid,
    note: NoteChangeset,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Note, crate::Error> {
    note.validate()?;

    match sqlx::query!(
        "UPDATE notes SET text = $1 WHERE id = $2 RETURNING id",
        note.text,
        id
    )
    .fetch_optional(executor)
    .await
    .map_err(crate::Error::DbError)?
    {
        Some(record) => Ok(Note {
            id: record.id,
            text: note.text,
        }),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn delete(
    id: Uuid,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<(), crate::Error> {
    match sqlx::query!("DELETE FROM notes WHERE id = $1 RETURNING id", id)
        .fetch_optional(executor)
        .await
        .map_err(crate::Error::DbError)?
    {
        Some(_) => Ok(()),
        None => Err(crate::Error::NoRecordFound),
    }
}
