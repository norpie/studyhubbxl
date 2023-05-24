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
    Err(UserError::InternalError)
}

pub fn scope() -> Scope {
    web::scope("/filters").service(get_filters)
}
