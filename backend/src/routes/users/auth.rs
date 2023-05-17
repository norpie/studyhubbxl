use actix_web::HttpResponse;
use actix_web::{web::Json, Responder, post};
use serde::Deserialize;

use crate::models::ApiResponse;
use crate::email;

#[derive(Debug, Deserialize)]
struct RegisterUser{
    email: String,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginUser{
    username: String,
    password: String,
}

//Request to register user
#[post("/user/register")]
async fn post_register_user(user: Json<RegisterUser>) -> impl Responder {
    ApiResponse::new("")
}

async fn register_user() -> HttpResponse{
    let _ = email::send_email("destination@destination.com", "Sign up", "Verify your email");
    HttpResponse::Ok().body("Sign up")
}

//Request to login user
#[post("/user/login")]
async fn post_login_user(user: Json<LoginUser>) -> impl Responder{
    ApiResponse::new("")
}

