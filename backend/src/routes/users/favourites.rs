use actix_web::{get, Responder, web::Json, post};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::ApiResponse;

#[derive(Debug,Deserialize)]
struct Favourite{
    id: Uuid,
}

#[derive(Debug,Deserialize)]
struct ListFavourites{
    top: Option<u8>,
    skip: Option<u8>,
    coordinates_only: Option<bool>,
}

//Request to set location to favourite
#[post("/user/favourite")]
async fn post_favorite(id: Json<Favourite>) -> impl Responder{
    ApiResponse::new(" ")
}

//Get list of favourites
#[get("/favourites")]
async fn get_favorites(list_favourites: Json<ListFavourites>) -> impl Responder{
    ApiResponse::new(" ")
}
