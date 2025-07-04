use crate::routes::init_routes;
use crate::state::AppState;
use axum::{
    body::{Body, Bytes},
    http::{Method, Request},
    response::Response,
    Router,
};
use forge_api_config::{load_config, Config, Environment};
use forge_api_db::{
    test_helpers::{setup_db, teardown_db},
    DbPool,
};
use hyper::header::{HeaderMap, HeaderName};
use std::cell::OnceCell;
use tower::ServiceExt;

/// A request that a test sends to the application.
///
/// TestRequests are constructed via the test context (see[`DbTestContext`]).
///
/// Example:
/// ```
/// let response = context
///     .app
///     .request("/greet")
///     .method(Method::GET)
///     .send()
///     .await;
/// ```
pub struct TestRequest {
    router: Router,
    uri: String,
    method: Method,
    headers: HeaderMap,
    body: Body,
}

impl TestRequest {
    fn new(router: Router, uri: &str) -> Self {
        Self {
            router,
            uri: String::from(uri),
            headers: HeaderMap::new(),
            body: Body::empty(),
            method: Method::GET,
        }
    }

    /// Sets the HTTP method for the request, e.g. GET or POST, see [`axum::http::Method`].
    #[allow(unused)]
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    /// Adds an HTTP header to the request.
    ///
    /// Header names must be passed as [`hyper::header::HeaderName`] while values can be passed as [`&str`]s.
    ///
    /// Example:
    /// ```
    /// let response = context
    ///     .app
    ///     .request("/greet")
    ///     .method(Method::GET)
    ///     .header(.header(http::header::CONTENT_TYPE, "application/json"))
    ///     .await;
    /// ```
    #[allow(unused)]
    pub fn header(mut self, name: HeaderName, value: &str) -> Self {
        self.headers.insert(name, value.parse().unwrap());
        self
    }

    /// Sets the body for the request.
    ///
    /// Example:
    /// ```
    /// let response = context
    ///     .app
    ///     .request("/tasks")
    ///     .method(Method::POST)
    ///     .body(Body::from(json!({
    ///         "description": "get milk!",
    ///     }).to_string()))
    /// ```
    #[allow(unused)]
    pub fn body(mut self, body: Body) -> Self {
        self.body = body;
        self
    }

    /// Sends the request to the application under test.
    #[allow(unused)]
    pub async fn send(self) -> Response {
        let mut request_builder = Request::builder().uri(&self.uri);

        for (key, value) in &self.headers {
            request_builder = request_builder.header(key, value);
        }

        request_builder = request_builder.method(&self.method);

        let request = request_builder.body(self.body);

        self.router.oneshot(request.unwrap()).await.unwrap()
    }
}

/// Testing convenience functions for [`axum::Router`].
pub trait RouterExt {
    /// Creates a [`TestRequest`] pointed at the application under test.
    #[allow(unused)]
    fn request(&self, uri: &str) -> TestRequest;
}

impl RouterExt for Router {
    #[allow(unused)]
    fn request(&self, uri: &str) -> TestRequest {
        TestRequest::new(self.clone(), uri)
    }
}

/// Testing convenience functions for [`axum::body::Body`].
pub trait BodyExt {
    /// Returns the body as raw bytes.
    #[allow(unused, async_fn_in_trait)]
    async fn into_bytes(self) -> Bytes;

    /// Returns the body as parsed JSON.
    ///
    /// Example:
    /// ```
    /// let response = context
    ///     .app
    ///     .request("/tasks")
    ///     .method(Method::GET)
    ///     .send()
    ///     .await;
    ///
    /// let tasks: Vec<Task> = response.into_body().into_json::<Vec<Task>>().await;
    /// ```
    #[allow(unused, async_fn_in_trait)]
    async fn into_json<T>(self) -> T
    where
        T: serde::de::DeserializeOwned;
}

impl BodyExt for Body {
    #[allow(unused)]
    async fn into_bytes(self) -> Bytes {
        // We don't care about the size limit in tests.
        axum::body::to_bytes(self, usize::MAX)
            .await
            .expect("Failed to read response body")
    }

    #[allow(unused)]
    async fn into_json<T>(self) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        let body = self.into_bytes().await;
        serde_json::from_slice::<T>(&body).expect("Failed to deserialize JSON body")
    }
}
/// Provides context information for application tests.
///
/// A `DbTestContext` is passed as an argument to tests marked with the [`forge_api_macros::db_test`] attribute macro. It is used to access the application under test as well as the database (which is the same database the application under test uses).
///
/// Example:
/// ```
/// #[db_test]
/// async fn test_read_all(context: &DbTestContext) {
///     let task_changeset: TaskChangeset = Faker.fake();
///     create_task(task_changeset.clone(), &context.db_pool)
///         .await
///         .unwrap();
///
///     let response = context
///         .app
///         .request("/tasks")
///         .method(Method::GET)
///         .send()
///         .await;
///
///     assert_that!(response.status(), eq(StatusCode::OK));
///
///     let tasks: TasksList = response.into_body().into_json::<TasksList>().await;
///     assert_that!(tasks, len(eq(1)));
///     assert_that!(
///         tasks.first().unwrap().description,
///         eq(task_changeset.description)
///     );
/// }
/// ```
pub struct DbTestContext {
    /// The application that is being tested.
    pub app: Router,
    /// A connection pool connected to the same database that the application that is being tested uses as well.
    pub db_pool: DbPool,
}

/// Sets up a test and returns a [`DbTestContext`] configured for the particular test case.
///
/// This function initializes a new instance of the application under test using the configuration for [`forge_api_config::Environment::Test`]. The application is configured to use the same database that is also made available to the test itself via the test context. That database is a clone of the main test database that is only used by the particular test case to ensure isolation between test cases. It is automatically torn down after the test case completes (see [`teardown`]).
///
/// This function is not invoked directly but used inside of the [`forge_api_macros::db_test`] attribute macro. The test context is automatically passed to test cases marked with that macro as an argument.
#[allow(unused)]
pub async fn setup() -> DbTestContext {
    let init_config: OnceCell<Config> = OnceCell::new();
    let config = init_config.get_or_init(|| load_config(&Environment::Test).unwrap());

    let test_db_pool = setup_db(&config.database).await;

    let app = init_routes(AppState {
        db_pool: test_db_pool.clone(),
    });

    DbTestContext {
        app,
        db_pool: test_db_pool,
    }
}

/// Tears down a [`DbTestContext`].
///
/// This function drops the test-case specific database set up by [`setup`].
///
/// This function is not invoked directly but used inside of the [`forge_api_macros::db_test`] attribute macro. The test context is automatically passed to test cases marked with that macro as an argument.
#[allow(unused)]
pub async fn teardown(context: DbTestContext) {
    drop(context.app);

    teardown_db(context.db_pool);
}
