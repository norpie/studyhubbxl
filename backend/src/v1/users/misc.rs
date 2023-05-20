use actix_web::{
    get,
    web::{self, Path},
    Responder, Scope,
};
use uuid::Uuid;

use crate::models::ApiResponse;
use crate::error::MyResult;

//Get reset password page
#[get("/reset/{id}")]
async fn reset_password(id: Path<Uuid>) -> impl Responder {
    MyResult::Ok(ApiResponse::new(" "))
}

//Get delete user page
#[get("/delete/{id}")]
async fn delete_account(id: Path<Uuid>) -> impl Responder {
    MyResult::Ok(ApiResponse::new(" "))
}

pub fn scope() -> Scope {
    web::scope("")
        .service(delete_account)
        .service(reset_password)
}
