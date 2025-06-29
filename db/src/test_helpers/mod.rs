use crate::{connect_pool, DbPool};
use forge_api_config::DatabaseConfig;
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use regex::{Captures, Regex};
use sqlx::postgres::{PgConnectOptions, PgConnection};
use sqlx::{Connection, Executor};
use std::str::FromStr;
use std::sync::Arc;

/// All test functionality related to the [`crate::entities::users::User`] entity
pub mod users;

/// Sets up a dedicated database to be used in a test case.
///
/// This sets up a dedicated database as a fork of the main test database as configured in `.env.test`. The database can be used in a test case to ensure the test case is isolated from other test cases. The function returns a connection pool connected to the created database.
/// This function is automatically called by the [`forge-api-macros::db_test`] macro. The return connection pool is passed to the test case via the [`forge-api-macros::DbTestContext`].
#[allow(unused)]
pub async fn setup_db(config: &DatabaseConfig) -> DbPool {
    let test_db_config = prepare_db(config).await;
    connect_pool(test_db_config)
        .await
        .expect("Could not connect to database!")
}

/// Drops a dedicated database for a test case.
///
/// This function is automatically called by the [`forge-api-macros::db_test`] macro. It ensures test-specific database are cleaned up after each test run so we don't end up with large numbers of unused databases.
pub async fn teardown_db(db_pool: DbPool) {
    let mut connect_options = db_pool.connect_options();
    let db_config = Arc::make_mut(&mut connect_options);

    drop(db_pool);

    let root_db_config = db_config.clone().database("postgres");
    let mut connection: PgConnection = Connection::connect_with(&root_db_config).await.unwrap();

    let test_db_name = db_config.get_database().unwrap();

    let query = format!("DROP DATABASE IF EXISTS {}", test_db_name);
    connection.execute(query.as_str()).await.unwrap();
}

async fn prepare_db(config: &DatabaseConfig) -> DatabaseConfig {
    let db_config = parse_db_config(&config.url);
    let db_name = db_config.get_database().unwrap();

    let root_db_config = db_config.clone().database("postgres");
    let mut connection: PgConnection = Connection::connect_with(&root_db_config).await.unwrap();

    let test_db_name = build_test_db_name(db_name);

    let query = format!("CREATE DATABASE {} TEMPLATE {}", test_db_name, db_name);
    connection.execute(query.as_str()).await.unwrap();

    let regex = Regex::new(r"(.+)\/(.+$)").unwrap();
    let test_db_url = regex.replace(&config.url, |caps: &Captures| {
        format!("{}/{}", &caps[1], test_db_name)
    });

    DatabaseConfig {
        url: test_db_url.to_string(),
    }
}

fn build_test_db_name(base_name: &str) -> String {
    let test_db_suffix: String = rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    format!("{}_{}", base_name, test_db_suffix).to_lowercase()
}

fn parse_db_config(url: &str) -> PgConnectOptions {
    PgConnectOptions::from_str(url).expect("Invalid DATABASE_URL!")
}
