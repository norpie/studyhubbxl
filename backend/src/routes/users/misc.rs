use actix_web::{web::Path, Responder, get};
use uuid::Uuid;

use crate::models::ApiResponse;

//Get reset password page
#[get("/user/reset/{id}")]
async fn reset_password(id: Path<Uuid>) -> impl Responder{
    ApiResponse::new(" ")
}

//Get delete user page
#[get("/user/delete/{id}")]
async fn delete_account(id: Path<Uuid>) -> impl Responder{
    ApiResponse:: new(" ")
}


