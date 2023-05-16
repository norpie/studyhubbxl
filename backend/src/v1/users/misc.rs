use actix_web::{web::{Path, self}, Responder, get, Scope};
use uuid::Uuid;

use crate::models::ApiResponse;

//Get reset password page
#[get("/reset/{id}")]
async fn reset_password(id: Path<Uuid>) -> impl Responder{
    ApiResponse::new(" ")
}

//Get delete user page
#[get("/delete/{id}")]
async fn delete_account(id: Path<Uuid>) -> impl Responder{
    ApiResponse:: new(" ")
}


pub fn scope() -> Scope {
    web::scope("/misc").service(reset_password).service(delete_account)
}