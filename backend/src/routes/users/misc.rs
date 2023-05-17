use actix_web::{web::Path, Responder, get, HttpResponse};
use uuid::Uuid;

use crate::models::ApiResponse;
use crate::email;

//Get reset password page
#[get("/user/reset/{id}")]
async fn reset_password(id: Path<Uuid>) -> impl Responder{
    ApiResponse::new(" ")
}

async fn password_reset() -> HttpResponse{
    let _ = email::send_email("destination@destination.com", "Password reset", "Change your password and repeat it");
    HttpResponse::Ok().body("Password reset")
}

//Get delete user page
#[get("/user/delete/{id}")]
async fn delete_account(id: Path<Uuid>) -> impl Responder{
    ApiResponse:: new(" ")

}

async fn account_deletion() -> HttpResponse{
    let _ = email::send_email("destination@destination.com", "Account deletion", "We are sorry to see you go, but you have 30 days to change your mind!");
    HttpResponse::Ok().body("Account deletion")
}



