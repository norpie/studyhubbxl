use actix_web::{get, Responder, web::{Query, Json, Path}, post, HttpRequest, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::ApiResponse;

#[derive(Debug, Deserialize)]
 struct LocationQuery{
    search: String,
    top: Option<u8>,
    skip: Option<u8>,
    coordinates_only: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct Filter{
    location_types: Vec<String>,
    attributes: Vec<String>,
    noise: Vec<String>,
}


//Request locations by search or filter or both
#[post("/locations")]
async fn post_location(query: Query<LocationQuery>, filter: Json<Filter>, request: HttpRequest) -> impl Responder {
    //Check the access token of the user with the ones corresponding to our database (surreal db)
    if let Some(token) = request.headers().get("Authorization"){
           let token = token.to_str().unwrap();
            if token = "on of our tokens"{
                let response = ApiResponse::new(" ");
              return HttpResponse::Ok().json(response)
            
           }else{
            return HttpResponse::Unauthorized().body("Invalid token/No token")
           }
    }
   
}

//Get specific location
#[get("/location/{id}")]
async fn get_location(id: Path<Uuid>, request: HttpRequest) -> impl Responder{
     //Check the access token of the user with the ones corresponding to our database (surreal db)
     if let Some(token) = request.headers().get("Authorization"){
        let token = token.to_str().unwrap();
         if token = "on of our tokens"{
             let response = ApiResponse::new(" ");
           return HttpResponse::Ok().json(response)
         
        }else{
         return HttpResponse::Unauthorized().body("Invalid token/No token")
        }
 }

}
