use actix_web::{
    body::BoxBody,
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse, Responder, ResponseError,
};
use derive_more::{Display, Error};

use crate::models::ApiResponse;

#[derive(Debug, Display, Error)]
pub enum UserError {
    #[display(fmt = "A validation error occurred on field: {}", field)]
    ValidationError { field: String },
    #[display(fmt = "An internal error occurred. Please try again later")]
    InternalError,
    #[display(fmt = "Wrong password/username")]
    WrongPasswordOrUsername,
    #[display(fmt = "Too many requests")]
    TooManyRequests,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::WrongPasswordOrUsername => StatusCode::UNAUTHORIZED,
            UserError::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
        }
    }
}

impl Responder for UserError {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        ApiResponse::<String>::error(self.status_code().as_u16(), self.to_string()).respond_to(req)
    }

    //fn respond_to(self, _: &actix_web::HttpRequest) -> Self::Future{
    //    match self {
    //        Ok(api_response) =>{
    //            let response = HttpResponse::Ok()
    //            .insert_header(("Access-Control-Allow-Origin", "http://localholst::8080"))
    //            .content_type(ContentType::json())
    //            .body(serde_json::to_string(&api_response).unwrap());
    //        actix_web::dev::ready(Ok(response))
    //        }
    //        Err(user_error) => {
    //            let code = user_error.status_code().as_u16();
    //            let msg = user_error.to_string();
    //            let error_response = ApiResponse::error(code as u8, msg);
    //            let response = HttpResponse::build(user_error.status_code())
    //                .insert_header(ContentType.json())
    //                .body(serde_json::to_string(&error_response).unwrap());
    //            actix_web::dev::ready(Ok(response))
    //        }
    //    }
    //}

}
 pub type ResultWithError<T> = Result<ApiResponse<T>, UserError>;
