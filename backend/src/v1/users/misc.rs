use actix_web::{
    get,
    web::{self, Data},
    HttpRequest, Scope,
};
use chrono::Utc;
use surrealdb::{engine::remote::ws::Client, Surreal};
use uuid::Uuid;

use crate::{
    email,
    models::{ApiResponse, User},
};
use crate::{
    error::{Result, UserError},
    models::DeleteOrReset,
};

//Get reset password page
#[get("/reset")]
async fn reset_password(
    db: Data<Surreal<Client>>,
    req: HttpRequest,
) -> Result<ApiResponse<&'static str>> {
    let id = super::parse_id(req).await?;
    let slug = sha256::digest(Uuid::new_v4().to_string());
    let result: surrealdb::Result<DeleteOrReset> = db
        .create("reset")
        .content(DeleteOrReset {
            identifier: id.clone(),
            generation_time: Utc::now(),
            slug: slug.clone(),
        })
        .await;
    match result {
        Ok(_) => {
            let query_result = db
                .query("SELECT * FROM user WHERE identifier = $id")
                .bind(("id", id))
                .await;
            match query_result {
                Ok(mut response) => {
                    let take_result: surrealdb::Result<Option<User>> = response.take(0);
                    match take_result {
                        Ok(optional_user) => {
                            let user = optional_user.unwrap();
                            let email_result = email::send_email(
                                &user.email,
                                "Password Reset",
                                &("https://ourlink.domain/reset/".to_string() + &slug),
                            )
                            .await;
                            match email_result {
                                Ok(_) => Ok(ApiResponse::new("")),
                                Err(err) => {
                                    println!("error: {:#?}", &err);
                                    Err(UserError::InternalError)
                                }
                            }
                        }
                        Err(err) => {
                            println!("error: {:#?}", &err);
                            Err(UserError::InternalError)
                        }
                    }
                }
                Err(err) => {
                    println!("error: {:#?}", &err);
                    Err(UserError::InternalError)
                }
            }
        }
        Err(err) => {
            println!("error: {:#?}", &err);
            Err(UserError::InternalError)
        }
    }
}

//Get delete user page
#[get("/delete")]
async fn delete_account(
    db: Data<Surreal<Client>>,
    req: HttpRequest,
) -> Result<ApiResponse<&'static str>> {
    let id = super::parse_id(req).await?;
    let slug = sha256::digest(Uuid::new_v4().to_string());
    let result: surrealdb::Result<DeleteOrReset> = db
        .create("udelete")
        .content(DeleteOrReset {
            identifier: id.clone(),
            generation_time: Utc::now(),
            slug: slug.clone(),
        })
        .await;
    match result {
        Ok(_) => {
            let query_result = db
                .query("SELECT * FROM user WHERE identifier = $id")
                .bind(("id", id))
                .await;
            match query_result {
                Ok(mut response) => {
                    let take_result: surrealdb::Result<Option<User>> = response.take(0);
                    match take_result {
                        Ok(optional_user) => {
                            let user = optional_user.unwrap();
                            let email_result = email::send_email(
                                &user.email,
                                "Undelete Account",
                                &("https://ourlink.domain/reset/".to_string() + &slug),
                            )
                            .await;
                            match email_result {
                                Ok(_) => Ok(ApiResponse::new("")),
                                Err(err) => {
                                    println!("error: {:#?}", &err);
                                    Err(UserError::InternalError)
                                }
                            }
                        }
                        Err(err) => {
                            println!("error: {:#?}", &err);
                            Err(UserError::InternalError)
                        }
                    }
                }
                Err(err) => {
                    println!("error: {:#?}", &err);
                    Err(UserError::InternalError)
                }
            }
        }
        Err(err) => {
            println!("error: {:#?}", &err);
            Err(UserError::InternalError)
        }
    }
}

pub fn scope() -> Scope {
    web::scope("")
        .service(delete_account)
        .service(reset_password)
}
