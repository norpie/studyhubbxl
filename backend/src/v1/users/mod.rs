use std::str::FromStr;

use actix_web::{web, HttpRequest, Scope};
use surrealdb::{engine::remote::ws::Client, Surreal};
use uuid::Uuid;

use crate::{
    error::{Result, UserError},
    models::Session,
};

mod auth;
mod favourites;
mod misc;

pub fn scope() -> Scope {
    web::scope("/users")
        .service(favourites::scope())
        .service(misc::scope())
        .service(auth::scope())
}

// TODO: implement
pub async fn parse_id(db: &Surreal<Client>, req: HttpRequest) -> Result<Uuid> {
    if let Some(cookie) = req.cookie("session") {
        let uuid_result = Uuid::from_str(cookie.value());
        match uuid_result {
            Ok(session) => {
                let query_result = db
                    .query("SELECT * FROM session WHERE session_id = $id LIMIT 1")
                    .bind(("id", session))
                    .await;
                match query_result {
                    Ok(mut response) => {
                        let session_result: surrealdb::Result<Option<Session>> = response.take(0);
                        match session_result {
                            Ok(optional_session) => {
                                if let Some(session) = optional_session {
                                    Ok(session.identifier)
                                } else {
                                    Err(UserError::Unathorized)
                                }
                            }
                            Err(_) => Err(UserError::Unathorized),
                        }
                    }
                    Err(_) => Err(UserError::Unathorized),
                }
            }
            Err(_) => Err(UserError::Unathorized),
        }
    } else {
        Err(UserError::Unathorized)
    }
}
