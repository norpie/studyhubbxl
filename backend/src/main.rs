use actix_web::{get, web, App, HttpServer, Responder};
use auth::{Request, State};
use v1::{location, users};

use crate::models::ApiResponse;

mod auth;
mod models;
mod v1;

#[get("/")]
async fn hello() -> impl Responder {
    ApiResponse::new("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/private")
                    .service(users::scope())
                    .wrap(Request::new(State::Private)),
            )
            .service(
                web::scope("/public")
                    .service(location::scope())
                    .wrap(Request::new(State::Public)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
