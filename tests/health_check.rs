//! tests/health_check.rs

// LLO pg 34
// Author runs the test on this page and it passes, but I'm getting this failure message:

// ---- health_check_works stdout ----
// thread 'health_check_works' panicked at 'Failed to execute request.: reqwest::Error { kind: Request, url: Url { scheme: "http", cannot_be_a_base: false, username: "", password: None, host: Some(Ipv4(127.0.0.1)), port: Some(5000), path: "/health_check", query: None, fragment: None }, source: hyper::Error(Connect, ConnectError("tcp connect error", Os { code: 61, kind: ConnectionRefused, message: "Connection refused" })) }', tests/health_check.rs:61:10
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// ^^ NOTE: This line:
// spawn_app().await;
// in health_check_works/0 is still required.
// The book says .await is no longer necessary, but the test fails without it.

// use zero2prod::main; <- After adding the lib.rs file and making changes to Cargo.toml, this is no longer necessary.
// ^^ This initially complains with:
// use of undeclared crate or module `zero2prod`.

// We need to refactor our project into a library and a binary:
// - All business logic will live in the library crate
// - The binary itself will just be an entrypoint with a very slim main function.

// Change Cargo.toml from:
// [package]
// name = "zero2prod"
// version = "0.1.0"
// edition = "2018"

// [dependencies]
// actix-web = "=4.0.0-beta.8"
// actix-http = "=3.0.0-beta.8"

// To:
// [package]
// name = "zero2prod"
// version = "0.1.0"
// edition = "2018"

// [lib]
// # Any path may be used here, but this is a community convention.
// path = "src/lib.rs"

// # Not absolutely necessary, but helps remove ambiguity once we move away from the default configuration.
// # Note the double brackets, it's an array in TOML syntax.
// # We can only have one library, but we can have multiple binaries.
// [[bin]]
// path = "src/main.rs"
// name = "zero2prod"

// [dependencies]
// actix-web = "=4.0.0-beta.8"
// actix-http = "=3.0.0-beta.8"

// #[test]
// fn dummy_test() {
//     main()
// } <- Also removed after the changes to Cargo.toml

// You can inspect what code gets generated using `cargo expand --test health_check` <- name of the test file
#[actix_rt::test] // The test equivalent of `actix_web::main`
async fn health_check_works() {
    spawn_app().await; // .await.expect("Failed to spawn our app."); <- No longer necessary after (Refactor #1)

    // Use `cargo add reqest --dev --vers 0.11` to add it under [dev-dependencies] in Cargo.toml
    // ^^ add doesn't seem to be available as a command.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    // assert_eq!(Some(0), response.content_length());
}

// Launch the Application in the background
// spawn_app is the only piece of test code that will depend on our application code.
// Everything else is decoupled from the underlying implementation details.
// If we decided to rewrite the application in Ruby on Rails, we could still use the same test suite
// to check for regressions in our new stack as long as spawn_app gets replaced with the appropriate trigger (e.g. a bash command to launch the Rails app)
async fn spawn_app() { // -> std::io::Result<()> { <- Removed after (Refactor #1)
    // tokia::spawn takes a future and hands it over to the runtime for polling, without waiting for its completion; it therefore
    // runs concurrently with downstream futures and tasks (e.g. our test logic).
    // ^^ Will require a refactor in lib.rs (Refactor #1)
    let server = zero2prod::run().expect("Failed to bind address");

    // Add tokio as a new dev-dependency
    let _ = tokio::spawn(server);
}