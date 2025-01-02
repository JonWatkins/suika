use std::fmt;

/// Represents various HTTP errors that can occur.
#[derive(Debug)]
pub enum HttpError {
    /// Bad Request (400) error with a specific message.
    BadRequest(String),
    /// Unauthorized (401) error with a specific message.
    Unauthorized(String),
    /// Forbidden (403) error with a specific message.
    Forbidden(String),
    /// Not Found (404) error with a specific message.
    NotFound(String),
    /// Internal Server Error (500) with a specific message.
    InternalServerError(String),
}

impl fmt::Display for HttpError {
    /// Formats the `HttpError` for display.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::error::HttpError;
    ///
    /// let error = HttpError::BadRequest("Invalid data".to_string());
    /// assert_eq!(format!("{}", error), "Bad Request: Invalid data");
    ///
    /// let error = HttpError::NotFound("Resource missing".to_string());
    /// assert_eq!(format!("{}", error), "Not Found: Resource missing");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            HttpError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            HttpError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            HttpError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            HttpError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
        }
    }
}

impl std::error::Error for HttpError {}

impl HttpError {
    /// Converts the `HttpError` into a tuple containing the status code and the error message.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::error::HttpError;
    ///
    /// let error = HttpError::BadRequest("Invalid data".to_string());
    /// assert_eq!(error.to_tuple(), (400, "Invalid data"));
    ///
    /// let error = HttpError::NotFound("Resource missing".to_string());
    /// assert_eq!(error.to_tuple(), (404, "Resource missing"));
    /// ```
    pub fn to_tuple(&self) -> (u16, &str) {
        match self {
            HttpError::BadRequest(msg) => (400, msg),
            HttpError::Unauthorized(msg) => (401, msg),
            HttpError::Forbidden(msg) => (403, msg),
            HttpError::NotFound(msg) => (404, msg),
            HttpError::InternalServerError(msg) => (500, msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let bad_request = HttpError::BadRequest("Invalid data".to_string());
        let unauthorized = HttpError::Unauthorized("No token".to_string());
        let forbidden = HttpError::Forbidden("Access denied".to_string());
        let not_found = HttpError::NotFound("Resource missing".to_string());
        let internal_server_error =
            HttpError::InternalServerError("Server malfunction".to_string());

        assert_eq!(format!("{}", bad_request), "Bad Request: Invalid data");
        assert_eq!(format!("{}", unauthorized), "Unauthorized: No token");
        assert_eq!(format!("{}", forbidden), "Forbidden: Access denied");
        assert_eq!(format!("{}", not_found), "Not Found: Resource missing");
        assert_eq!(
            format!("{}", internal_server_error),
            "Internal Server Error: Server malfunction"
        );
    }

    #[test]
    fn test_to_tuple() {
        let bad_request = HttpError::BadRequest("Invalid data".to_string());
        let unauthorized = HttpError::Unauthorized("No token".to_string());
        let forbidden = HttpError::Forbidden("Access denied".to_string());
        let not_found = HttpError::NotFound("Resource missing".to_string());
        let internal_server_error =
            HttpError::InternalServerError("Server malfunction".to_string());

        assert_eq!(bad_request.to_tuple(), (400, "Invalid data"));
        assert_eq!(unauthorized.to_tuple(), (401, "No token"));
        assert_eq!(forbidden.to_tuple(), (403, "Access denied"));
        assert_eq!(not_found.to_tuple(), (404, "Resource missing"));
        assert_eq!(
            internal_server_error.to_tuple(),
            (500, "Server malfunction")
        );
    }
}
