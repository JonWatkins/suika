use std::fmt;

#[derive(Debug)]
pub enum HttpError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    InternalServerError(String),
}

impl fmt::Display for HttpError {
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
    fn test_http_error_display() {
        let error = HttpError::BadRequest("Invalid request data".to_string());
        assert_eq!(format!("{}", error), "Bad Request: Invalid request data");

        let error = HttpError::Unauthorized("Authentication required".to_string());
        assert_eq!(
            format!("{}", error),
            "Unauthorized: Authentication required"
        );

        let error = HttpError::Forbidden("Access denied".to_string());
        assert_eq!(format!("{}", error), "Forbidden: Access denied");

        let error = HttpError::NotFound("Resource not found".to_string());
        assert_eq!(format!("{}", error), "Not Found: Resource not found");

        let error = HttpError::InternalServerError("Internal server error".to_string());
        assert_eq!(
            format!("{}", error),
            "Internal Server Error: Internal server error"
        );
    }

    #[test]
    fn test_http_error_to_tuple() {
        let error = HttpError::BadRequest("Invalid request data".to_string());
        let (code, msg) = error.to_tuple();
        assert_eq!(code, 400);
        assert_eq!(msg, "Invalid request data");

        let error = HttpError::Unauthorized("Authentication required".to_string());
        let (code, msg) = error.to_tuple();
        assert_eq!(code, 401);
        assert_eq!(msg, "Authentication required");

        let error = HttpError::Forbidden("Access denied".to_string());
        let (code, msg) = error.to_tuple();
        assert_eq!(code, 403);
        assert_eq!(msg, "Access denied");

        let error = HttpError::NotFound("Resource not found".to_string());
        let (code, msg) = error.to_tuple();
        assert_eq!(code, 404);
        assert_eq!(msg, "Resource not found");

        let error = HttpError::InternalServerError("Internal server error".to_string());
        let (code, msg) = error.to_tuple();
        assert_eq!(code, 500);
        assert_eq!(msg, "Internal server error");
    }
}
