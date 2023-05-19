use actix_web::{Scope, web};

mod auth;
mod favourites;
mod misc;

pub fn scope() -> Scope {
    web::scope("/users")
        .service(favourites::scope())
        .service(misc::scope())
        .service(auth::scope())
}
