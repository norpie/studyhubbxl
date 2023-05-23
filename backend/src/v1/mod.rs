use actix_web::{web, Scope};

mod location;
mod users;

pub fn public() -> Scope {
    web::scope("").service(location::scope())
}

pub fn private() -> Scope {
    web::scope("").service(users::scope())
}
