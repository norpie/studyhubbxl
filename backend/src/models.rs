use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Wrapper for every response made by the backend
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    status: u16,
}

impl<T: Serialize> ApiResponse<T> {
    /// Wrap object in ApiResponse
    pub fn new(object: T) -> Self {
        ApiResponse {
            result: Some(object),
            status: 200,
            error: None,
        }
    }

    /// Wrap error in ApiResponse
    pub fn error(status: u16, error: String) -> Self {
        ApiResponse {
            error: Some(error),
            status,
            result: None,
        }
    }
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let result = serde_json::to_string(&self);
        match result {
            Ok(json) => HttpResponse::Ok()
                .insert_header(("Access-Control-Allow-Origin", "http://localhost:5173"))
                .content_type(ContentType::json())
                .body(json),
            Err(_e) => {
                // TODO: log
                HttpResponse::InternalServerError().body("")
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    email: String,
    username: String,
    password: String,
    salt: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
enum LocationType {
    Cafe,
    Library,
    StudySpace,
    Campus,
}

#[derive(Debug, Serialize, Deserialize)]
enum Attribute {
    Sockets,
    Wifi,
    CoWorking,
}

#[derive(Debug, Serialize, Deserialize)]
enum Noise {
    Noisy,
    Moderate,
    Quiet,
    Silent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    id: Uuid,
    name: String,
    location_type: LocationType,
    attributes: Vec<Attribute>,
    noise: Noise,
    address: String,
    coordinates: (Decimal, Decimal),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Favourite {
    id: Uuid,
    location_id: Uuid,
    user_id: Uuid,
}
