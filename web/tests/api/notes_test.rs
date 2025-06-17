use axum::{
    body::Body,
    http::{self, Method},
};
use fake::{Fake, Faker};
use forge_api_db::test_helpers::users::{create as create_user, UserChangeset};
use forge_api_db::{entities, transaction, Error};
use forge_api_macros::db_test;
use forge_api_web::test_helpers::{BodyExt, DbTestContext, RouterExt};
use googletest::prelude::*;
use hyper::StatusCode;

use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

#[db_test]
async fn test_create_invalid(context: &DbTestContext) {
    let payload = json!(entities::notes::NoteChangeset {
        text: String::from("")
    });

    let response = context
        .app
        .request("/notes")
        .method(Method::POST)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::UNPROCESSABLE_ENTITY));
}

#[db_test]
async fn test_create_success(context: &DbTestContext) {
    let changeset: entities::notes::NoteChangeset = Faker.fake();
    let payload = json!(changeset);

    let response = context
        .app
        .request("/notes")
        .method(Method::POST)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::CREATED));

    let notes = entities::notes::load_all(&context.db_pool).await.unwrap();
    assert_that!(notes, len(eq(1)));
    assert_that!(notes.first().unwrap().text, eq(&changeset.text));
}

#[db_test]
async fn test_read_all(context: &DbTestContext) {
    let changeset: entities::notes::NoteChangeset = Faker.fake();
    entities::notes::create(changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let response = context.app.request("/notes").send().await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let notes: Vec<entities::notes::Note> = response
        .into_body()
        .into_json::<Vec<entities::notes::Note>>()
        .await;
    assert_that!(notes, len(eq(1)));
    assert_that!(notes.first().unwrap().text, eq(&changeset.text));
}

#[db_test]
async fn test_read_one_nonexistent(context: &DbTestContext) {
    let response = context
        .app
        .request(&format!("/notes/{}", Uuid::new_v4()))
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::NOT_FOUND));
}

#[db_test]
async fn test_read_one_success(context: &DbTestContext) {
    let note_changeset: entities::notes::NoteChangeset = Faker.fake();
    let note = entities::notes::create(note_changeset.clone(), &context.db_pool)
        .await
        .unwrap();
    let note_id = note.id;

    let response = context
        .app
        .request(&format!("/notes/{}", note_id))
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let note: entities::notes::Note = response
        .into_body()
        .into_json::<entities::notes::Note>()
        .await;
    assert_that!(note.id, eq(note_id));
    assert_that!(note.text, eq(&note_changeset.text));
}

#[db_test]
async fn test_update_invalid(context: &DbTestContext) {
    let note_changeset: entities::notes::NoteChangeset = Faker.fake();
    let note = entities::notes::create(note_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let payload = json!(entities::notes::NoteChangeset {
        text: String::from("")
    });

    let user_changeset: UserChangeset = Faker.fake();
    create_user(user_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let response = context
        .app
        .request(&format!("/notes/{}", note.id))
        .method(Method::PUT)
        .body(Body::from(payload.to_string()))
        .header(http::header::AUTHORIZATION, &user_changeset.token)
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::UNPROCESSABLE_ENTITY));

    let note_after = entities::notes::load(note.id, &context.db_pool)
        .await
        .unwrap();
    assert_that!(note_after.text, eq(&note.text));
}

#[db_test]
async fn test_update_nonexistent(context: &DbTestContext) {
    let note_changeset: entities::notes::NoteChangeset = Faker.fake();
    let payload = json!(note_changeset);

    let user_changeset: UserChangeset = Faker.fake();
    create_user(user_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let response = context
        .app
        .request(&format!("/notes/{}", Uuid::new_v4()))
        .method(Method::PUT)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .header(http::header::AUTHORIZATION, &user_changeset.token)
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::NOT_FOUND));
}

#[db_test]
async fn test_update_success(context: &DbTestContext) {
    let note_changeset: entities::notes::NoteChangeset = Faker.fake();
    let note = entities::notes::create(note_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let note_changeset: entities::notes::NoteChangeset = Faker.fake();
    let payload = json!(note_changeset);

    let user_changeset: UserChangeset = Faker.fake();
    create_user(user_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let response = context
        .app
        .request(&format!("/notes/{}", note.id))
        .method(Method::PUT)
        .header(http::header::AUTHORIZATION, &user_changeset.token)
        .body(Body::from(payload.to_string()))
        .header(http::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::OK));

    let note: entities::notes::Note = response
        .into_body()
        .into_json::<entities::notes::Note>()
        .await;
    assert_that!(note.text, eq(&note_changeset.text.clone()));

    let note = entities::notes::load(note.id, &context.db_pool)
        .await
        .unwrap();
    assert_that!(note.text, eq(&note_changeset.text));
}

#[db_test]
async fn test_delete_nonexistent(context: &DbTestContext) {
    let user_changeset: UserChangeset = Faker.fake();
    create_user(user_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let response = context
        .app
        .request(&format!("/notes/{}", Uuid::new_v4()))
        .method(Method::DELETE)
        .header(http::header::AUTHORIZATION, &user_changeset.token)
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::NOT_FOUND));
}

#[db_test]
async fn test_delete_success(context: &DbTestContext) {
    let note_changeset: entities::notes::NoteChangeset = Faker.fake();
    let note = entities::notes::create(note_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let user_changeset: UserChangeset = Faker.fake();
    create_user(user_changeset.clone(), &context.db_pool)
        .await
        .unwrap();

    let response = context
        .app
        .request(&format!("/notes/{}", note.id))
        .method(Method::DELETE)
        .header(http::header::AUTHORIZATION, &user_changeset.token)
        .send()
        .await;

    assert_that!(response.status(), eq(StatusCode::NO_CONTENT));

    let result = entities::notes::load(note.id, &context.db_pool).await;
    assert_that!(result, err(anything()));
}
