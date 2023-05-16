use actix_web::{Scope, web};

mod favourites;
mod misc;
mod auth;

pub fn scope() -> Scope {
    web::scope("/users").service(favourites::scope()).service(misc::scope()).service(auth::scope())
}