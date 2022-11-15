use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    App, HttpResponse,
};
use derive_more::{Display, Error};


#[derive(Debug, Display, Error)]
enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "Bad Request")]
    BadRequest,
    #[display(fmt = "Not Found")]
    NotFound,
    #[display(fmt = "Timeout")]
    Timeout,
    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}