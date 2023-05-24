use std::collections::hash_map::IntoIter;

use actix_web::{
    get,
    web::{self, Data, Path},
    Scope,
};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    error::{Result, UserError},
    models::ApiResponse,
};

#[get("/{filter_type}")]
async fn get_filters(
    db: Data<Surreal<Client>>,
    filter_type: Path<String>,
) -> Result<ApiResponse<Vec<String>>> {
    match filter_type.as_str() {
        "noise" => Err(UserError::InternalError),
        "type" => Err(UserError::InternalError),
        "attributes" => Err(UserError::InternalError),
        _ => Err(UserError::InternalError),
    }
}

pub fn scope() -> Scope {
    web::scope("/filters").service(get_filters)
}
