use actix_web::{web::{Json, self}, Responder, post, Scope};
use serde::Deserialize;

use crate::models::ApiResponse;

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
#[post("/register")]
async fn post_register_user(user: Json<RegisterUser>) -> impl Responder {
    ApiResponse::new("")
}

//Request to login user
#[post("/login")]
async fn post_login_user(user: Json<LoginUser>) -> impl Responder{
    ApiResponse::new("")
}

pub fn scope() -> Scope {
    web::scope("/auth").service(post_register_user).service(post_login_user)
}