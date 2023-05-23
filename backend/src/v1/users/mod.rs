use std::str::FromStr;

use actix_web::{Scope, web, HttpRequest};
use uuid::Uuid;

use crate::error::Result;

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
pub fn parse_id(req: HttpRequest) -> Result<Uuid> {
    Ok(Uuid::from_str("b7c7295d-3cb9-40ec-8880-fe3c23577959").unwrap())
}
