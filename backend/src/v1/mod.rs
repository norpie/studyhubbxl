use actix_web::{Scope, web};

mod users;
mod location;

pub fn scope() -> Scope {
    web::scope("/v1").service(users::scope()).service(location::scope())
}