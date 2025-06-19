use serde::{Deserialize, Serialize};
use sqlx::Postgres;
use uuid::Uuid;

/// A user record.
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct User {
    /// The id of the record.
    pub id: Uuid,
    /// The user's name.
    pub name: String,
    pub pass: String,
    pub token: String,
}

/// Loads a user based on the passed token.
///
/// If no user exists for the token, [`Option::None`] is returned, otherwise `Option::Some(User)` is returned.
pub async fn load_with_token(
    token: &str,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Option<User>, anyhow::Error> {
    Ok(sqlx::query_as!(
        User,
        "SELECT id, name, pass, token FROM users WHERE token = $1",
        token
    )
    .fetch_optional(executor)
    .await?)
}

pub async fn find_user_by_name(
    name: &str,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Option<User>, anyhow::Error> {
    Ok(sqlx::query_as!(
        User,
        "SELECT id, name, pass, token FROM users WHERE name = $1",
        name
    )
    .fetch_optional(executor)
    .await?)
}

pub async fn insert_user(
    name: &str,
    hashed_pass: &str, // Password is already hashed
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<User, anyhow::Error> {
    // Validate inputs (optional but recommended)
    if name.is_empty() || hashed_pass.is_empty() {
        return Err(anyhow::anyhow!("Name and password must not be empty"));
    }
    let token = "random_text";
    // Insert the user and return the newly created record
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, pass, token)
        VALUES ($1, $2, $3)
        RETURNING id, name, pass, token
        "#,
        name,
        hashed_pass,
        &token
    )
    .fetch_one(executor)
    .await?;

    Ok(user)
}
