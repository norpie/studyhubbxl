use std::io::Error;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use limiter::RateLimiter;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

mod limiter;
mod models;
mod search;
mod error;

mod v1;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    //connection to server
    let result = Surreal::new::<Ws>("localhost:8000").await;
    if result.is_err() {
        panic!("No db connection!");
    }
    let db = result.unwrap();

    //Sign in as a namespace, database or root user
    let result = db
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await;
    if let Err(err) = result {
        panic!("{:#?}", err);
    }

    //Select specific namespace/database
    let result = db.use_ns("db").use_db("db").await;
    if let Err(err) = result {
        panic!("{:#?}", err);
    }
    HttpServer::new(move || {
        App::new().app_data(Data::new(db.clone())).service(
            web::scope("/api").service(
                web::scope("/v1")
                    .service(v1::private())
                    .service(v1::public())
            ),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
