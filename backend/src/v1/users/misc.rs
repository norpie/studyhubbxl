use actix_web::{
    get,
    web::{self, Path},
    Responder, Scope,
};
use uuid::Uuid;

use crate::models::ApiResponse;
use crate::error::Result;

//Get reset password page
#[get("/reset/{id}")]
async fn reset_password(id: Path<Uuid>) -> Result<ApiResponse<& 'static str>> {
    let bool = true;
    if bool{
        return Ok(ApiResponse::new(""));
    }
    Err(crate::error::UserError::WrongPasswordOrUsername)
}

//Get delete user page
#[get("/delete/{id}")]
async fn delete_account(id: Path<Uuid>) -> Result<ApiResponse<& 'static str>> {
    let bool = true;
    if bool{
        return Ok(ApiResponse::new(""))
    }
    Err(crate::error::UserError::WrongPasswordOrUsername)
}

pub fn scope() -> Scope {
    web::scope("")
        .service(delete_account)
        .service(reset_password)
}
