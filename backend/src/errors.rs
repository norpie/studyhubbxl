use actix_web::{error, HttpResponse, http::{header::ContentType, StatusCode}};
use derive_more::{Display, Error};
    
#[derive(Debug, Display, Error)]
enum UserError{
    #[display(fmt = "A validation error occurred on field: {}", field)]
    ValidationError{ field: String},
    #[display(fmt = "An internal error occurred. Please try again later")]
    InternalError,
}

impl error::ResponseError for UserError{
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
            HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}