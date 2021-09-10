//! tests/health_check.rs
use std::net::TcpListener;

// SOLVED Error #1:
// pg 34
// Author runs the test on this page and it passes, but I'm getting this failure message:

// ---- health_check_works stdout ----
// thread 'health_check_works' panicked at 'Failed to execute request.: reqwest::Error { kind: Request, url: Url { scheme: "http", cannot_be_a_base: false, username: "", password: None, host: Some(Ipv4(127.0.0.1)), port: Some(5000), path: "/health_check", query: None, fragment: None }, source: hyper::Error(Connect, ConnectError("tcp connect error", Os { code: 61, kind: ConnectionRefused, message: "Connection refused" })) }', tests/health_check.rs:61:10
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// ^^ NOTE: This line:
// spawn_app().await;
// in health_check_works/0 is still required.
// The book says .await is no longer necessary, but the test fails without it.
// **************************************************************************

// **************************************************************************
// pg 38
// Error #2:
// Getting this error message after making the changes for (Refactor #3) and running `cargo test`:
// ---- health_check_works stdout ----
// thread 'health_check_works' panicked at 'Failed to execute request.: reqwest::Error { kind: Builder, source: EmptyHost }', tests/health_check.rs:91:10
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// empty host...
// But my WTF println!s check out:
// WTF
// "http:://127.0.0.1:63698/health_check"

// So reqwest's .get/1 method is being provided with a proper url path.

// Added a log to the main.rs function so that I can see what port the server is being started on, so that I may issue a manual curl request,
// and getting this error:
// curl -v http:://127.0.0.1:63702/health_check                                                                                              
// * Closing connection -1
// curl: (3) URL using bad/illegal format or missing URL

// ^^ IMMEDIATE TODO: Gonna need to do some google searching about this curl error.

// ^^ HAHA, after stepping away for a moment and coming back to this, found this github issue:
// https://github.com/seanmonstar/reqwest/issues/720#issuecomment-558687670
// Definitely overlooked that extra colon.
// Tests passing now.
// **************************************************************************

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
// actix_rt::test spins up a new runtime at the beginning of each test case and they shut down at the end of each test case.
// No need to implement any clean up logic to avoid leaking resources between test runs.
#[actix_rt::test] // The test equivalent of `actix_web::main`
async fn health_check_works() {
    // spawn_app().await; // .await.expect("Failed to spawn our app."); <- No longer necessary after (Refactor #1)

    // After (Refactor #3)
    // Yeah I don't understand why the book doesn't use await, the type without using .await is Future<Output = String>
    let address = spawn_app().await;

    // Use `cargo add reqest --dev --vers 0.11` to add it under [dev-dependencies] in Cargo.toml
    // ^^ add doesn't seem to be available as a command.
    let client = reqwest::Client::new();

    // Error #2 debugging
    println!("WTF");
    println!("{:#?}", format!("{}/health_check", &address));

    // Act
    let response = client
    // Now with the (Refactor #2) we need to find out what port the OS has gifted our application
    // and return it from spawn_app.
    // There are a few ways to go about doing this, we will use a std::net::TcpListener.
    // HttpServer is handling two steps when given an address:
    // 1. it will bind it
    // 2. and then it will start the application.
    // We will take over the first step, by binding the port on our own with TcpListener and then hand that over to the HttpServer using
    // the listen method.
    // TcpListener::local_addr returns a SocketAddr which exposes the actual port we bound via .port()
    // (Refactor #3) begins in lib.rs
        // .get("http://127.0.0.1:8000/health_check")
        .get(&format!("{}/health_check", &address)) 
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
// async fn spawn_app() { // -> std::io::Result<()> { <- Removed after (Refactor #1)
//     // tokia::spawn takes a future and hands it over to the runtime for polling, without waiting for its completion; it therefore
//     // runs concurrently with downstream futures and tasks (e.g. our test logic).
//     // ^^ Will require a refactor in lib.rs (Refactor #1)
//     let server = zero2prod::run("127.0.0.1:0").expect("Failed to bind address");

//     // Add tokio as a new dev-dependency
//     // When a tokio runtime is shut down all tasks spawned on it are dropped.
//     let _ = tokio::spawn(server);
// }

// (Refactor #3)
async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // Returning the application address to the caller
    format!("http://127.0.0.1:{}", port)
}

// An improvement to be made:
// spawn_app will always try to run our applixation on port 8000.
// If we try to run two or more tests in parallel only one of them will manage to bind the port, all others will fail.
// We want tests to run their background application on a random available port.
// (Refactor #2) Change the run function to take the application address as an argument instead of relying on a hard-coded value.
// We can use port 0 to find a random available port.
// Port 0 is special-cased at the OS level: trying to bind port 0 will trigger an OS scan for an available port which will then be boudn to the application.

// This will be moved to a different test file later:
#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // Act
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

// subscribe_returns_a_400_when_data_is_missing is an example of *table-driven test* also known as *parameterised test*.
// A limitation of this roll your own approach is that as soona s one test case fails,
// the execution stops and we do not know the outcome for the following test cases.
#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}