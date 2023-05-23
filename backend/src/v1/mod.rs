use actix_web::{web, Scope};

mod location;
mod users;

pub fn scope() -> Scope {
    web::scope("")
        .service(users::scope())
        .service(location::scope())
}
