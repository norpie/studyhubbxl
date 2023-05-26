use actix_web::{
    get,
    web::{self, Data},
    HttpRequest, Scope,
};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::error::{Result, UserError};
use crate::models::ApiResponse;

//Get reset password page
#[get("/reset/{id}")]
async fn reset_password(
    db: Data<Surreal<Client>>,
    req: HttpRequest,
) -> Result<ApiResponse<&'static str>> {
    let id = super::parse_id(req);
    Err(UserError::WrongPasswordOrUsername)
}

//Get delete user page
#[get("/delete/{id}")]
async fn delete_account(
    db: Data<Surreal<Client>>,
    req: HttpRequest,
) -> Result<ApiResponse<&'static str>> {
    let id = super::parse_id(req);
    Err(UserError::WrongPasswordOrUsername)
}

pub fn scope() -> Scope {
    web::scope("")
        .service(delete_account)
        .service(reset_password)
}
