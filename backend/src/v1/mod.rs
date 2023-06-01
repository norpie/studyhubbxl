use actix_web::{web, Scope};

mod filter;
mod location;
mod users;

pub fn scope() -> Scope {
    web::scope("/v1")
        .service(users::scope())
        .service(location::scope())
        .service(filter::scope())
}
