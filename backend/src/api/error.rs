use actix_web::{ResponseError, HttpResponse, http::StatusCode};
use serde::Deserialize;
use std::fmt;
use sqlx::Error as SqlxError;

use super::response::JsonApiResponse;

#[derive(Debug, Deserialize)]
pub struct HttpError {
    pub status_code: u16,
    pub message: String,
}

impl HttpError {
    /// Helper factory function to create a new HttpError
    ///
    /// Arguments:
    /// - status_code: The HTTP status code to return
    /// - message: The message for the user describing the error
    pub fn new(status_code: u16, message: String) -> HttpError {
        HttpError {
            status_code,
            message,
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

// Convert an SqlxError into an HttpError
impl From<SqlxError> for HttpError {
    fn from(error: SqlxError) -> HttpError {
        match error {
            SqlxError::Database(err) => HttpError::new(409, err.message().to_string()),
            SqlxError::PoolTimedOut => HttpError::new(408, "DB Pool timed out".to_string()),
            SqlxError::RowNotFound => {
                HttpError::new(404, "The record was not found".to_string())
            }
            err => HttpError::new(500, format!("Unexpected DB error: {}", err)),
        }
    }
}

// Implement the ResponseError trait to generate a JSON API response
impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => "Internal server error".to_string(),
        };

        HttpResponse::build(status_code)
            .json(JsonApiResponse::<String>::error(vec![error_message], None))
    }
}
