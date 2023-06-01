use actix_web::{
    get, post,
    web::{self, Data, Json, Path},
    Either, Scope,
};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use uuid::Uuid;

use crate::{
    error::Result,
    models::{Coordinates, Location},
};
use crate::{error::UserError, models::ApiResponse};

#[derive(Debug, Deserialize)]
struct Filter {
    search: String,
    limit: u8,
    start: u8,
    coordinates_only: bool,
    location_types: Vec<String>,
    attributes: Vec<String>,
    noise: Vec<String>,
}

//Request locations by search or filter or both
#[post("")]
async fn filter_search_locations(
    db: Data<Surreal<Client>>,
    filter: Json<Filter>,
) -> Result<Either<ApiResponse<Vec<Location>>, ApiResponse<Vec<Coordinates>>>> {
    let sql = "SELECT * FROM location WHERE noise IN $noise AND attributes CONTAINSALL $attributes AND location_type IN $location_types AND name ~ $search LIMIT $limit START $start";
    let mut query = db.query(sql);
    query = query.bind(("noise", &filter.noise));
    query = query.bind(("search", &filter.search));
    query = query.bind(("attributes", &filter.attributes));
    query = query.bind(("location_types", &filter.location_types));
    query = query.bind(("start", filter.start));
    query = query.bind(("limit", filter.limit));
    let query_result = query.await;
    match query_result {
        Ok(mut response) => {
            let parse_result: surrealdb::Result<Vec<Location>> = response.take(0);
            match parse_result {
                Ok(locations) => {
                    if filter.coordinates_only {
                        let mut coords = Vec::new();
                        for location in locations {
                            coords.push(location.coords());
                        }
                        Ok(Either::Right(ApiResponse::new(coords)))
                    } else {
                        Ok(Either::Left(ApiResponse::new(locations)))
                    }
                }
                Err(err) => {
                    println!("error inner: {:#?}", err);
                    Err(UserError::InternalError)
                }
            }
        }
        Err(err) => {
            println!("error outer: {:#?}", err);
            Err(UserError::InternalError)
        }
    }
}

//Get specific location
#[get("/{id}")]
async fn get_location(
    db: Data<Surreal<Client>>,
    id: Path<Uuid>,
) -> Result<ApiResponse<Option<Location>>> {
    let query_result = db
        .query("SELECT * FROM location WHERE '$id' LIMIT 1")
        .bind(("$id", id.to_string()))
        .await;
    match query_result {
        Ok(mut response) => {
            let parse_result = response.take(0);
            match parse_result {
                Ok(location) => Ok(ApiResponse::new(location)),
                Err(err) => {
                    println!("error inner: {:#?}", err);
                    Err(UserError::InternalError)
                }
            }
        }
        Err(err) => {
            println!("error outer: {:#?}", err);
            Err(UserError::InternalError)
        }
    }
}

pub fn scope() -> Scope {
    web::scope("/locations")
        .service(get_location)
        .service(filter_search_locations)
}
