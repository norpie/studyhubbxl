use actix_web::{
    get, post,
    web::{self, Data, Path},
    HttpRequest, Scope,
};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use uuid::Uuid;

use crate::error::Result;
use crate::models::ApiResponse;

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
    let id = super::parse_id(req)?;
    let query_result = db
        .query("CREATE favourite CONTENT { user_id: $user_id,  location_id: $location_id }")
        .bind(("user_id", id.to_string()))
        .bind(("location_id", location_id.to_string()))
        .await;
    match query_result {
        Ok(response) => Ok(ApiResponse::new("")),
        Err(err) => {
            println!("error outer: {:#?}", err);
            Err(crate::error::UserError::InternalError)
        }
    }
}

//Get list of favourites
#[get("")]
async fn get_favourites(
    list_favourites: Path<ListFavourites>,
) -> Result<ApiResponse<&'static str>> {
    Err(crate::error::UserError::InternalError)
}

pub fn scope() -> Scope {
    web::scope("/favourites")
        .service(get_favourites)
        .service(new_favourite)
}
