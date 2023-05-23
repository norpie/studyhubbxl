use actix_web::{
    get, post,
    web::{self, Json, Path, Query},
    Scope,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::ApiResponse;
use crate::{error::Result, models::Location};

#[derive(Debug, Deserialize)]
struct LocationQuery {
    search: Option<String>,
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
async fn filter_search_locations(
    query: Query<LocationQuery>,
    filter: Json<Filter>,
) -> Result<ApiResponse<Vec<Location>>> {
    return Ok(ApiResponse::new(Vec::new()));
}

//Get specific location
#[get("/{id}")]
async fn get_location(id: Path<Uuid>) -> Result<ApiResponse<Option<Location>>> {
    Ok(ApiResponse::new(None))
}

pub fn scope() -> Scope {
    web::scope("/locations")
        .service(get_location)
        .service(filter_search_locations)
}
