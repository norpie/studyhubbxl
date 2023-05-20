use actix_web::{
    get, post,
    web::{self, Json},
    Responder, Scope,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::ApiResponse;
use crate::error::Result;

#[derive(Debug, Deserialize)]
struct Favourite {
    id: Uuid,
}

#[derive(Debug, Deserialize)]
struct ListFavourites {
    top: Option<u8>,
    skip: Option<u8>,
    coordinates_only: Option<bool>,
}

//Request to set location to favourite
#[post("/")]
async fn new_favourite(id: Json<Favourite>) -> Result<ApiResponse<& 'static str>> {
    let bool = true;
    if bool{
        return Ok(ApiResponse::new(""))
    }
    Err(crate::error::UserError::InternalError)
}

//Get list of favourites
#[get("/")]
async fn get_favourites(list_favourites: Json<ListFavourites>) -> Result<ApiResponse<& 'static str>> {
    let bool = true;
    if bool{
        return Ok(ApiResponse::new(""))
    }
    Err(crate::error::UserError::InternalError)
}

pub fn scope() -> Scope {
    web::scope("/favourites")
        .service(get_favourites)
        .service(new_favourite)
}
