use actix_web::{
    post,
    web::{self, Data, Json},
    Scope,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use uuid::Uuid;

use crate::{error::Result, models::User};
use crate::{error::UserError, models::ApiResponse};

#[derive(Debug, Deserialize)]
struct RegisterUser {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

fn hash(password: String, salt: &SaltString) -> String {
    Argon2::default()
        .hash_password(password.as_bytes(), salt)
        .unwrap()
        .to_string()
}

//Request to register user
#[post("/register")]
async fn register_user(
    db: Data<Surreal<Client>>,
    input: Json<RegisterUser>,
) -> Result<ApiResponse<Uuid>> {
    // Check existing user
    let query_result = db
        .query("SELECT * FROM user WHERE email = $email")
        .bind(("email", &input.email))
        .await;
    match query_result {
        Ok(mut response) => {
            let user_result: surrealdb::Result<Option<User>> = response.take(0);
            match user_result {
                Ok(optional_user) => {
                    if optional_user.is_some() {
                        return Err(UserError::EmailUsed);
                    }
                }
                Err(_err) => {
                    //TODO: log
                    return Err(UserError::InternalError);
                }
            }
        }
        Err(_err) => {
            //TODO: log
            return Err(UserError::InternalError);
        }
    };
    // Make user
    let salt = SaltString::generate(&mut OsRng);
    let hash = hash(input.password.clone(), &salt);
    let id = Uuid::new_v4();
    let query_result = db
        .query("CREATE user CONTENT { id: $id, email: $email, password: $password, salt: $salt }")
        .bind(("id", &id))
        .bind(("email", &input.email))
        .bind(("password", &hash))
        .bind(("salt", &salt.to_string()))
        .await;
    if query_result.is_err() {
        return Err(UserError::InternalError);
    }
    // Make session
    let session_id = Uuid::new_v4();
    let query_result = db
        .query("CREATE session content { user_id: $user_id, session_id: $session_id }")
        .bind(("user_id", id))
        .bind(("session_id", session_id))
        .await;
    if query_result.is_err() {
        return Err(UserError::InternalError);
    }
    Ok(ApiResponse::new(session_id))
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
