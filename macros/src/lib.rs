//! The forge_api-macros crate contains the `test` and `db_test` macros.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[allow(clippy::test_attr_in_doctest)]
/// Used to mark an application test.
///
/// Example:
/// ```
/// #[test]
/// async fn test_hello(context: &TestContext) {
///     let response = context.app.request("/greet").send().await;
///
///     let greeting: Greeting = response.into_body().into_json().await;
///     assert_that!(greeting.hello, eq(String::from("world")));
/// }
/// ```
///
/// Test functions marked with this attribute receive a [`forge_api-web::test_helpers::TestContext`] struct via which they get access a preconfigured instance of the application. The application instance is extended with convenience methods for making requests from the test.
#[proc_macro_attribute]
pub fn test(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let test_attrs = input.attrs;
    let test_name = input.sig.ident.clone();
    let test_arguments = input.sig.inputs;
    let test_block = input.block;
    let inner_test_name = syn::Ident::new(
        format!("inner_{}", test_name).as_str(),
        input.sig.ident.span(),
    );

    let setup = quote! {
        let context = forge_api_web::test_helpers::setup().await;
    };

    let output = quote!(
        #[::tokio::test]
        #(#test_attrs)*
        async fn #test_name() {
            #setup
            async fn #inner_test_name(#test_arguments) #test_block
            #inner_test_name(&context).await;
        }
    );

    TokenStream::from(output)
}

/// Used to mark an application test.
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
///
/// Test functions marked with this attribute receive a [`forge_api-web::test_helpers::DbTestContext`] struct via which they get access a preconfigured instance of the application as well as a pool of database connections. The connection pool is connected to the dedicated database for this single test case. The application instance is configured to be connected to the same database so that data created in the test is accessible to the application and vice versa (see in the example how a task is created in the task, which the application reads and responds with as JSON). That allows full-stack testing without interfering with other tests.
///
/// The test-specific database is cleaned up automatically so that no manual cleanup is necessary.
#[proc_macro_attribute]
pub fn db_test(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let test_attrs = input.attrs;
    let test_name = input.sig.ident.clone();
    let test_arguments = input.sig.inputs;
    let test_block = input.block;
    let inner_test_name = syn::Ident::new(
        format!("inner_{}", test_name).as_str(),
        input.sig.ident.span(),
    );

    let setup = quote! {
        let context = forge_api_web::test_helpers::setup().await;
    };

    let teardown = quote! {
        forge_api_web::test_helpers::teardown(context).await;
    };

    let output = quote!(
        #[::tokio::test]
        #(#test_attrs)*
        async fn #test_name() {
            #setup
            async fn #inner_test_name(#test_arguments) #test_block
            #inner_test_name(&context).await;
            #teardown
        }
    );

    TokenStream::from(output)
}
