use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

// LLO pg 46

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

// async fn subscribe() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// ************************************************************

// The subscribe handler after using the Form Extractor (provided by actix-web)
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
// ^^ Immediately after implementing this, all of the subscribe_returns_a_400_when_data_is_missing parameterized tests pass...
// But why?
// Looking at the implementation of Form in actix-web is required to understand why.
// #[derive(PartialEq, Eq, PartialOrd, Ord)]
// pub struct Form<T>(pub T); <- Nothing more than a wrapper, a generic type T which is used to populate Form's only field.
// Where does the extraction take place?
// An extractor is a type that implements the FromRequest trait (its definition is a bit noisy because Rust doesn't yet support async fn in trait definitions).

// It looks more or less something like this:
// NOTE: Types that implement this trait can be used with `Route` handlers.
// pub trait FromRequest: Sized {
//     type Error = Into<actix_web::Error>;

//     async fn from_request(=
//         req: &HttpRequest,
//         payload: &mut Payload
//     ) -> Result<Self, Self::Error>;
// }

// from_request takes as inputs the head of the incoming HTTP request and the
// bytes of its payload.

// All arguments in the signature of a route handler must implement the FromRequest trait: actix-web
// will invoke from_request for each argument and, if the extraction succeeds for all of them, it will
// then run the actual handler function.
// If one of the extractions fails, the corresponding error is returned to the caller and the handler is never invoked
// (actix_web::Error can be converted to a HttpResponse).

// Here's what Form's FromRequest implementation roughly looks like:
// impl <T> FromRequest for Form<T>
// where
//     T: DeserializeOwned + 'static,
// {
//     type Error = actix_web::Error;

//     async fn from_request(
//         req: &HttpRequest,
//         payload: &mut Payload
//     ) -> Result<Self, Self::Error> {
//         // Omitted stuff around extractor configuration (e.g. payload size limits)

//         match UrlEncoded::new(req, payload).await {
//             Ok(item) => Ok(Form(item)),
//             // The error handler can be customized.
//             // The default one wil return a 400, which is what we want.
//             Err(e) => Err(error_handler(e))
//         }
//     }
// }

// UrlEncoded is handling a lot of the heavy-lifting.
// It transparently handles compressed and uncompressed payloads, it deals w/ the fact that
// the request body arrives a chunk at a time as a stream of bytes, etc.

// And finally, after all the above has been taken care of:
// serde_urlencoded::from_bytes::<T>(&body).map_err(|_| UrlencodedError::Parse)

// serde_urlencoded provides (de)serialization support for the application/x-www-form-urlencoded data format.

// from_bytes takes as input a contiguous slice of bytes and it deserializes an instance of type T from it according
// to the rules of the URL-encoded format.
// How does it know how to do it for a generic type T?
// It is because T implements the DeserializedOwned trait from serde.

// To understand what is *actually* going on under the hood we need to take a closer look at serde (which will touch upon some advanced Rust topics).

// LLO pg 46
// ************************************************************

// Also can be written as:
// async fn health_check() -> Responder {
//     HttpResponse::Ok().finish()
// }

// pub async fn run() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }

// (Refactor #1)
// pub fn run() -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| {
//             App::new()
//                 .route("/health_check", web::get().to(health_check))
//         })
//         .bind("127.0.0.1:8000")?
//         .run();

//     // println!("{:#?}", server);
//     Ok(server)
// }

// (Refactor #2)
// pub fn run(address: &str) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| {
//             App::new()
//                 .route("/health_check", web::get().to(health_check))
//         })
//         .bind(address)?
//         .run();

//     // println!("{:#?}", server);
//     Ok(server)
// }

// (Refactor #3)
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run();

    // println!("{:#?}", server);
    Ok(server)
}