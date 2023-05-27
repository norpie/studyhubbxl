use actix_web::{
    post,
    web::{self, Json},
    Scope,
};
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng, Salt}};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::Result;
use crate::models::ApiResponse;

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

fn hash(password: String, salt: Salt) -> String {
    Argon2::default().hash_password(password.as_bytes(), salt).unwrap().to_string()
}

//Request to register user
#[post("/register")]
async fn register_user(input: Json<RegisterUser>) -> Result<ApiResponse<&'static str>> {
    Err(crate::error::UserError::WrongPasswordOrUsername)

}

//Request to login user
#[post("/login")]
async fn login_user(input: Json<LoginUser>) -> Result<ApiResponse<&'static str>> {
    Err(crate::error::UserError::WrongPasswordOrUsername)
}

pub fn scope() -> Scope {
    web::scope("/auth")
        .service(login_user)
        .service(register_user)
}
