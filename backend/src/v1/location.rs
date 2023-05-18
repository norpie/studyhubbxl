use actix_web::{
    get, post,
    web::{Json, Path, Query, self},
    Responder, Scope,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::ApiResponse;

#[derive(Debug, Deserialize)]
struct LocationQuery {
    search: String,
    top: Option<u8>,
    skip: Option<u8>,
    coordinates_only: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct Filter {
    location_types: Vec<String>,
    attributes: Vec<String>,
    noise: Vec<String>,
}

//Request locations by search or filter or both
#[post("/")]
async fn filter_search_locations(query: Query<LocationQuery>, filter: Json<Filter>) -> impl Responder {
    ApiResponse::new(" ")
}

//Get specific location
#[get("/{id}")]
async fn get_location(id: Path<Uuid>) -> impl Responder {
    ApiResponse::new(" ")
}

pub fn scope() -> Scope {
    web::scope("/locations")
        .service(get_location)
        .service(filter_search_locations)
}
