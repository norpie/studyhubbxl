use actix_web::{error, HttpResponse, http::{header::ContentType, StatusCode}};
use derive_more::{Display, Error};
    
#[derive(Debug, Display, Error)]
pub enum UserError{
    #[display(fmt = "A validation error occurred on field: {}", field)]
    ValidationError{ field: String},
    #[display(fmt = "An internal error occurred. Please try again later")]
    InternalError,
    #[display(fmt = "Wrong password")]
    WrongPassword,
    #[display(fmt = "Too many requests")]
    TooManyRequests,
}

impl error::ResponseError for UserError{
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
            HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::WrongPassword => StatusCode:: UNAUTHORIZED,
            UserError::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
        }
    }
}