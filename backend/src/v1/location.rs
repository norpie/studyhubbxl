use actix_web::{
    get, post,
    web::{self, Data, Json, Path, Query},
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
struct Search {
    search: Option<String>,
    limit: Option<u8>,
    start: Option<u8>,
    coordinates_only: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct Filter {
    location_types: Vec<String>,
    attributes: Vec<String>,
    noise: Vec<String>,
}

//Request locations by search or filter or both
#[post("")]
async fn filter_search_locations(
    db: Data<Surreal<Client>>,
    search: Query<Search>,
    filter: Json<Filter>,
) -> Result<Either<ApiResponse<Vec<Location>>, ApiResponse<Vec<Coordinates>>>> {
    let start = if let Some(start) = search.start {
        start
    } else {
        0
    };
    let limit = if let Some(limit) = search.limit {
        limit
    } else {
        20
    };
    let search_string = if let Some(search) = &search.search {
        search.clone()
    } else {
        "".to_string()
    };
    let sql = "SELECT * FROM location WHERE noise IN $noise AND attributes CONTAINSALL $attributes AND location_type IN $location_types AND name ~ $search LIMIT $limit START $start";
    let mut query = db.query(sql);
    query = query.bind(("noise", &filter.noise));
    query = query.bind(("search", &search_string));
    query = query.bind(("attributes", &filter.attributes));
    query = query.bind(("location_types", &filter.location_types));
    query = query.bind(("start", start));
    query = query.bind(("limit", limit));
    let query_result = query.await;
    match query_result {
        Ok(mut response) => {
            let parse_result: surrealdb::Result<Vec<Location>> = response.take(0);
            match parse_result {
                Ok(locations) => {
                    if search.coordinates_only.is_some() && search.coordinates_only.unwrap() {
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
