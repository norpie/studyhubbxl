use actix_web::{get, Responder, web::{Json, self}, post, Scope};
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
#[post("/")]
async fn post_favourite(id: Json<Favourite>) -> impl Responder{
    ApiResponse::new(" ")
}

//Get list of favourites
#[get("/")]
async fn get_favourites(list_favourites: Json<ListFavourites>) -> impl Responder{
    ApiResponse::new(" ")
}
pub fn scope() -> Scope {
    web::scope("/favourites").service(get_favourites).service(post_favourite)
}