//! src/lib.rs
use actix_web::dev::Server;
use actix_web::HttpResponse;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    println!("{:#?}", req);
    let name = req.match_info().get("name").unwrap_or("User");
    format!("Hello {}!", &name)
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // HttpServer handles all transport level concerns
    let server = HttpServer::new(|| {
        // App - where all application logic lives; routing, middlewares, request handlers, etc
        // Takes care of incoming request as input and sends out a response
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
