use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// LLO pg 26

// Handler functions may have different function signatures.
// This is enabled via traits in actix_web.
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

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
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
