use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
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
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
        })
        .bind("127.0.0.1:8000")?
        .run();

    // println!("{:#?}", server);
    Ok(server)
}