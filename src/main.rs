//! src/main.rs
use actix_web::HttpResponse;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};


async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    println!("{:#?}",req);
    let name = req.match_info().get("name").unwrap_or("User");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // HttpServer handles all transport level concerns
    HttpServer::new(|| {
        // App - where all application logic lives; routing, middlewares, request handlers, etc
        // Takes care of incoming request as input and sends out a response
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
