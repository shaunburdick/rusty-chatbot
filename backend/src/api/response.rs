use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JsonApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<T>>,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<T>>,
}

/// Factory methods to create a success JSON API Response and a failure JSON API Response
impl<T> JsonApiResponse<T> {
    /// Produce a JSON API response indicating a success
    ///
    /// Arguments:
    /// - data: A Vec of data to return, can be empty
    /// - message: An optional message to include in the response
    pub fn success(data: Vec<T>, message: Option<String>) -> Self {
        Self {
            data: Some(data),
            message: message.unwrap_or("OK".to_string()),
            errors: None
        }
    }

    /// Produce a JSON API response indicating an error
    ///
    /// Arguments:
    /// - error: The errors to show
    /// - message; An optional message to include in the response
    pub fn error(errors: Vec<T>, message: Option<String>) -> Self {
        Self {
            data: None,
            message: message.unwrap_or("NOT OK".to_string()),
            errors: Some(errors),
        }
    }
}
