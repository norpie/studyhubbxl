use actix_web::{web, Scope};

use crate::auth::{AuthMiddleware, Request};

pub mod location;
pub mod users;

pub fn scope() -> Scope {
    web::scope("/v1")
}
