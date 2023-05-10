use actix_web::{web::Json, Responder, post};
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
#[post("/user/register")]
async fn post_register_user(user: Json<RegisterUser>) -> impl Responder {
    ApiResponse::new("")
}

//Request to login user
#[post("/user/login")]
async fn post_login_user(user: Json<LoginUser>) -> impl Responder{
    ApiResponse::new("")
}