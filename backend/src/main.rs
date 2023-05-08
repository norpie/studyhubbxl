use actix_web::{get, App, HttpServer, Responder};

use crate::models::ApiResponse;

mod models {
    use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
    use serde::Serialize;

    /// Wrapper for every response made by the backend
    #[derive(Debug, Clone, Serialize)]
    pub struct ApiResponse<T> {
        result: Option<T>,
        status: u8,
        error: String,
    }

    impl<T: Serialize> ApiResponse<T> {
        /// Wrap object in ApiResponse
        pub fn new(object: T) -> Self {
            ApiResponse {
                result: Some(object),
                status: 200,
                error: "".into(),
            }
        }

        /// Wrap error in ApiResponse
        pub fn error(status: u8, error: String) -> Self {
            ApiResponse {
                result: None,
                status,
                error,
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
}

#[get("/")]
async fn hello() -> impl Responder {
    ApiResponse::new("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
