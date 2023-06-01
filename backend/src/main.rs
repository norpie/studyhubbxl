use actix_cors::Cors;
use actix_web::{
    http, middleware,
    web::{self, Data},
    App, HttpServer,
};
use chrono::Utc;
use limiter::RateLimiter;
use models::DeleteOrReset;
use std::time::Duration;

use std::io::Error;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tokio::{task, time::sleep};

mod email;
mod error;
mod limiter;
mod models;

mod v1;

async fn get_db() -> Surreal<Client> {
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
    db
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let db = get_db().await;

    //Check accounts to delete (after 30 days, check every 10 mins)

    task::spawn(async move {
        let other_db = get_db().await;
        loop {
            let query = other_db.select("udelete").await;
            if query.is_err() {
                continue;
            }
            let deletes: Vec<DeleteOrReset> = query.unwrap();
            let mut ids = Vec::new();
            for delete in deletes {
                if Utc::now() - delete.generation_time > chrono::Duration::days(30) {
                    ids.push(delete.identifier);
                }
            }
            other_db.query("BEGIN TRANSACTION;");
            for user in ids {
                other_db.query("DELETE udelete WHERE identifier = $identifier; DELETE users WHERE identifier = $identifier;")
                .bind(("identifier", user)).await.ok();
            }
            other_db.query("COMMIT TRANSACTION;");

            sleep(Duration::new(600, 0)).await;
        }
    });
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .supports_credentials()
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .app_data(Data::new(db.clone()))
            .wrap(cors)
            .service(web::scope("/api").service(web::scope("/v1").service(v1::scope())))

        /*.wrap(RateLimiter::new())*/
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
