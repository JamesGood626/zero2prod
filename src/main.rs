use zero2prod::run;

// // Handler functions may have different function signatures.
// // This is enabled via traits in actix_web.
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

// // Responder is nothing more than a conversion trait into a HttpResponse.
// // The argument may be omitted, thanks to some type magic going on behind the scenes in actix-web.
// async fn health_check() -> impl Responder {
//     // HttpResponse::Ok().finish()
//     // HttpResponsebUilder implements Responder... so we can shorten the above to:
//     HttpResponse::Ok()
// }

// main cannot be an asynchronous function.
// All futures expose a poll method which has to be called to allow the future to make progress and eventually resolve to a final value.
// Rust's standard library doesn't include an asynchronous runtime by design (you provide your own as a dependency, and there's a Actor lib Bastion or Fuschia project or Tokio).
// This explains why main cannot be an asynchronous function: who is in charge to call poll on it?
// So you're expected to launch your asynchronous runtime at the top of the main function and then use it to drive your futures forward.

// #[actix_web::main] is a procedural macro. cargo-expand can be used to expand all macros in your code to help demystify it.
// cargo install cargo-expand
// running cargo expand initially failed with:
// error: the option `Z` is only accepted on the nightly compiler
// the github repo for cargo-expand says it'll find the nightly compiler.
// Guess it's possible that the nightly compiler isn't installed by default via rustup installation:
// https://stackoverflow.com/a/48675452
// rustup install nightly


// After creating the lib.rs file, we move this function to lib.rs, renaming it to run,
// and then just invoke run from main.rs.
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(greet))
//             .route("/{name}", web::get().to(greet))
//             .route("/health_check", web::get().to(health_check))
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address.
    // Otherwise call .await on our server.
    run()?.await
}

// For testing, there are three options:
// 1. Next to your code in an embedded test module (hidden behind a configuration conditional check, #[cfg(test)])
// 2. In an external tests folder
// 3. doctests

// An embedded test module has privleged access to the code living next to it:
// It can interact with structs, methods, fields and fucntions that have not been marked as public and would
// normally not be available to a user of our code if they were to import it as a dependency of their own project.

// So for a project with a minimal exposed Public API, you can integration test that with tests in an external folder
// and then use embedded test modules for the unit tests on the private sub-components.

// Tests in the external tests folder and doc tests, instead, have exactly the same level of access to your code
// that you would get if you were to add your crate as a dependency in another project (so they are mostly used
// for integration testing).
// Anything under the tests directory ends up being compiled in its own binary.


