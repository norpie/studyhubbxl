use actix_web::{error, HttpResponse, http::{header::ContentType, StatusCode}, ResponseError, Responder};
use derive_more::{Display, Error};


use crate::models::ApiResponse;
    
#[derive(Debug, Display, Error)]
pub enum UserError{
    #[display(fmt = "A validation error occurred on field: {}", field)]
    ValidationError{ field: String},
    #[display(fmt = "An internal error occurred. Please try again later")]
    InternalError,
    #[display(fmt = "Wrong password")]
    WrongPasswordOrUsername,
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
            UserError::WrongPasswordOrUsername => StatusCode:: UNAUTHORIZED,
            UserError::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
        }
    }
}

impl <T, Serialize> Responder for Result<ApiResponse<T>, UserError>{
    type Error = actix_web::Error;
    type Future = actix_web::dev::Ready<Result<HttpResponse, Self::Error>>;

    fn respond_to(self, _: &actix_web::HttpRequest) -> Self::Future{
        match self {
            Ok(api_response) =>{
                let response = HttpResponse::Ok()
                .insert_header(("Access-Control-Allow-Origin", "http://localholst::8080"))
                .content_type(ContentType::json())
                .body(serde_json::to_string(&api_response).unwrap());
            actix_web::dev::ready(Ok(response))
            }
            Err(user_error) => {
                let code = user_error.status_code().as_u16();
                let msg = user_error.to_string();
                let error_response = ApiResponse::error(code as u8, msg);
                let response = HttpResponse::build(user_error.status_code())
                    .insert_header(ContentType.json())
                    .body(serde_json::to_string(&error_response).unwrap());
                actix_web::dev::ready(Ok(response))
            }
        }
    }

   
}