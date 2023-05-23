use actix_web::{
    get, post,
    web::{self, Data, Json, Path, Query},
    Scope,
};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use uuid::Uuid;

use crate::{error::Result, models::Location};
use crate::{error::UserError, models::ApiResponse};

#[derive(Debug, Deserialize)]
struct Search {
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
    db: Data<Surreal<Client>>,
    search: Query<Search>,
    filter: Json<Filter>,
) -> Result<ApiResponse<Vec<Location>>> {
    let query = db.query("");
    return Ok(ApiResponse::new(Vec::new()));
}

//Get specific location
#[get("/{id}")]
async fn get_location(
    db: Data<Surreal<Client>>,
    id: Path<Uuid>,
) -> Result<ApiResponse<Option<Location>>> {
    let query_result = db
        .query("SELECT * FROM location WHERE '$id'")
        .bind(("$id", id.to_string()))
        .await;
    match query_result {
        Ok(mut response) => {
            let parse_result = response.take(0);
            match parse_result {
                Ok(location) => Ok(ApiResponse::new(location)),
                Err(err) => Err(UserError::InternalError),
            }
        }
        Err(err) => Err(UserError::InternalError),
    }
}

pub fn scope() -> Scope {
    web::scope("/locations")
        .service(get_location)
        .service(filter_search_locations)
}
