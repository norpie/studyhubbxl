use actix_web::{
    post,
    web::{self, Json},
    Responder, Scope,
};
use serde::Deserialize;

use crate::models::ApiResponse;
use crate::error::Result;

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
async fn register_user(user: Json<RegisterUser>) -> Result<ApiResponse<& 'static str>>{
    let bool = true;
    if bool{
        return Ok(ApiResponse::new(""))
    }
    Err(crate::error::UserError::WrongPasswordOrUsername)
}

//Request to login user
#[post("/login")]
async fn login_user(user: Json<LoginUser>) -> Result<ApiResponse<& 'static str>>{
    let bool = true;
    if bool{
        return Ok(ApiResponse::new(""))
    }
    Err(crate::error::UserError::WrongPasswordOrUsername)
}

pub fn scope() -> Scope {
    web::scope("/auth")
        .service(login_user)
        .service(register_user)
}
