use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JsonApiResponse<T> {
    pub data: Vec<T>,
    pub message: String,
    pub error: Option<String>,
}

impl<T> JsonApiResponse<T> {
    pub fn success(data: Vec<T>, message: Option<String>) -> Self {
        Self {
            data,
            message: message.unwrap_or("OK".to_string()),
            error: None
        }
    }

    pub fn error(error: String, message: Option<String>) -> Self {
        Self {
            data: Vec::new(),
            message: message.unwrap_or("OK".to_string()),
            error: Some(error),
        }
    }
}
