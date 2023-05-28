use actix_web::{
    get,
    web::{self, Data, Path},
    Scope,
};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    error::{Result, UserError},
    models::{ApiResponse, FilterItem},
};

#[get("/{filter_type}")]
async fn get_filters(
    db: Data<Surreal<Client>>,
    filter_type: Path<String>,
) -> Result<ApiResponse<Vec<FilterItem>>> {
    match filter_type.as_str() {
        "attribute" | "noise" | "location_type" => {
            let query_result = db
                .query("SELECT * FROM type::table($table);")
                .bind(("table", filter_type.to_string()))
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
        _ => Err(UserError::Unimplemented),
    }
}

pub fn scope() -> Scope {
    web::scope("/filters").service(get_filters)
}
