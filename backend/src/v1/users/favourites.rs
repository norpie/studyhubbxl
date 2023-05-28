use actix_web::{
    delete, get, post,
    web::{self, Data, Path, Query},
    Either, HttpRequest, Scope,
};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use uuid::Uuid;

use crate::models::{ApiResponse, Coordinates};
use crate::{
    error::{Result, UserError},
    models::Location,
};

#[derive(Debug, Deserialize)]
struct ListFavourites {
    limit: Option<u8>,
    start: Option<u8>,
    coordinates_only: Option<bool>,
}

//Request to set location to favourite
#[post("/{id}")]
async fn new_favourite(
    db: Data<Surreal<Client>>,
    location_id: Path<Uuid>,
    req: HttpRequest,
) -> Result<ApiResponse<&'static str>> {
    let id = super::parse_id(req).await?;
    let query_result = db
        .query("CREATE favourite CONTENT { user_id: $user_id,  location_id: $location_id }")
        .bind(("user_id", id.to_string()))
        .bind(("location_id", location_id.to_string()))
        .await;
    match query_result {
        Ok(_) => Ok(ApiResponse::new("")),
        Err(err) => {
            println!("error outer: {:#?}", err);
            Err(crate::error::UserError::InternalError)
        }
    }
}

#[delete("/{id}")]
async fn del_favourite(
    db: Data<Surreal<Client>>,
    location_id: Path<Uuid>,
    req: HttpRequest,
) -> Result<ApiResponse<&'static str>> {
    let id = super::parse_id(req).await?;
    let query_result = db
        .query("DELETE favourite WHERE user_id = $user_id AND location_id = $location_id;")
        .bind(("user_id", id.to_string()))
        .bind(("location_id", location_id.to_string()))
        .await;
    match query_result {
        Ok(_) => Ok(ApiResponse::new("")),
        Err(err) => {
            println!("error outer: {:#?}", err);
            Err(crate::error::UserError::InternalError)
        }
    }
}

//Get list of favourites
#[get("")]
async fn get_favourites(
    db: Data<Surreal<Client>>,
    query: Query<ListFavourites>,
    req: HttpRequest,
) -> Result<Either<ApiResponse<Vec<Location>>, ApiResponse<Vec<Coordinates>>>> {
    let id = super::parse_id(req).await?;
    let limit = if let Some(limit) = query.limit {
        limit
    } else {
        20
    };
    let start = if let Some(start) = query.start {
        start
    } else {
        0
    };
    let query_result = db
        .query("SELECT identifier, name, coordinates, address, attributes, location_type, noise FROM (SELECT *, (SELECT * FROM favourite WHERE user_id = $user_id LIMIT $limit START $start) as favourite FROM location SPLIT favourite) WHERE favourite.location_id = identifier;")
        .bind(("user_id", id.to_string()))
        .bind(("limit", limit))
        .bind(("start", start))
        .await;
    match query_result {
        Ok(mut response) => {
            let parse_result: surrealdb::Result<Vec<Location>> = response.take(0);
            match parse_result {
                Ok(locations) => {
                    if query.coordinates_only.is_some() && query.coordinates_only.unwrap() {
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

pub fn scope() -> Scope {
    web::scope("/favourites")
        .service(get_favourites)
        .service(del_favourite)
        .service(new_favourite)
}
