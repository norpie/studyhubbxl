use actix_web::{
    post,
    web::{self, Json},
    Responder, Scope,
};
use serde::Deserialize;

use crate::models::ApiResponse;
use crate::error::MyResult;

#[derive(Debug, Deserialize)]
struct RegisterUser {
    email: String,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

//Request to register user
#[post("/register")]
async fn register_user(user: Json<RegisterUser>) -> impl Responder {
   MyResult::Ok(ApiResponse::new(""))
}

//Request to login user
#[post("/login")]
async fn login_user(user: Json<LoginUser>) -> impl Responder {
    MyResult::Ok(ApiResponse::new(""))
    
}

pub fn scope() -> Scope {
    web::scope("/auth")
        .service(login_user)
        .service(register_user)
}
