use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    HttpResponse, HttpResponseBuilder, Responder,
};
use chrono::{DateTime, Utc};
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
    #[serde(skip_serializing)]
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
            Ok(json) => HttpResponseBuilder::new(StatusCode::from_u16(self.status).unwrap())
                .insert_header(("Access-Control-Allow-Origin", "http://localhost:5173"))
                .insert_header(("Access-Control-Allow-Credentials", "true"))
                .content_type(ContentType::json())
                .body(json),
            Err(_e) => {
                // TODO: log
                HttpResponse::InternalServerError()
                    .insert_header(("Access-Control-Allow-Origin", "http://localhost:5173"))
                    .insert_header(("Access-Control-Allow-Credentials", "true"))
                    .body("")
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub salt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterItem {
    identifier: String,
    path: String,
    display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub identifier: Uuid,
    pub name: String,
    pub location_type: String,
    pub attributes: Vec<String>,
    pub noise: String,
    pub address: String,
    pub lat: Decimal,
    pub long: Decimal,
}

impl Location {
    pub fn coords(&self) -> Coordinates {
        Coordinates {
            identifier: self.identifier,
            lat: self.lat,
            long: self.long,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub identifier: Uuid,
    pub lat: Decimal,
    pub long: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOrReset {
    pub identifier: Uuid,
    pub slug: String,
    pub generation_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ip {
    pub ip: String,
    pub window_start: DateTime<Utc>,
    pub requests: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Favourite {
    id: Uuid,
    location_id: Uuid,
    user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub user_id: Uuid,
    pub session_id: Uuid,
}
