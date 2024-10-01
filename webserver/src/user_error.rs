use actix_http::StatusCode;
use actix_web::{ResponseError, HttpResponse, http::header::ContentType};
use derive_more::Error;

#[derive(Debug, Error, PartialEq)]
pub enum UserError {
    InternalError
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::InternalError => write!(f, "{}", "An internal error occurred. Please try again later.")
        }
    }
}

impl UserError {
    pub fn from_sqlx_to_user_error(e: sqlx::Error) -> UserError {
        match e {
            sqlx::Error::RowNotFound => UserError::InternalError,
            sqlx::Error::ColumnDecode { .. } => UserError::InternalError,
            sqlx::Error::Decode(_) => UserError::InternalError,
            sqlx::Error::PoolTimedOut => UserError::InternalError,
            sqlx::Error::PoolClosed => UserError::InternalError,
            sqlx::Error::WorkerCrashed => UserError::InternalError,
            _ => UserError::InternalError,
        }
    }
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl Into<UserError> for sqlx::Error {
    fn into(self) -> UserError {
        UserError::from_sqlx_to_user_error(self)
    }
}
