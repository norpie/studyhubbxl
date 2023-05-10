use actix_web::{get, Responder, web::{Query, Json, Path}, post};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::ApiResponse;

#[derive(Debug, Deserialize)]
 struct LocationQuery{
    search: String,
    top: Option<u8>,
    skip: Option<u8>,
    coordinates_only: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct Filter{
    location_types: Vec<String>,
    attributes: Vec<String>,
    noise: Vec<String>,
}


//Request locations by search or filter or both
#[post("/locations")]
async fn post_location(query: Query<LocationQuery>, filter: Json<Filter>) -> impl Responder {
    ApiResponse::new(" ")
}

//Get specific location
#[get("/location/{id}")]
async fn get_location(id: Path<Uuid>) -> impl Responder{
    ApiResponse::new(" ")
}



