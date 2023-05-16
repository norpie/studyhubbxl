use actix_web::{get, App, HttpServer, Responder};

use crate::models::ApiResponse;

mod models;
mod v1;
mod auth;

#[get("/")]
async fn hello() -> impl Responder {
    ApiResponse::new("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(v1::scope()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
