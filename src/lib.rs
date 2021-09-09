use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}

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