use crate::HttpError;
use suika_mime::get_mime_type;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Result, Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Response {
    inner: Arc<Mutex<ResponseInner>>,
}

#[derive(Debug)]
struct ResponseInner {
    status_code: u16,
    headers: HashMap<String, String>,
    body: Option<Body>,
}

#[derive(Debug)]
enum Body {
    Text(String),
    Binary(Vec<u8>),
}

impl Response {
    pub fn new() -> Self {
        Response {
            inner: Arc::new(Mutex::new(ResponseInner {
                status_code: 200,
                headers: HashMap::new(),
                body: None,
            })),
        }
    }

    pub fn set_status(&self, code: u16) {
        let mut inner = self.inner.lock().unwrap();
        inner.status_code = code;
    }

    pub fn header(&self, key: &str, value: &str) {
        let mut inner = self.inner.lock().unwrap();
        inner.headers.insert(key.to_string(), value.to_string());
    }

    pub fn body(&self, body: String) {
        let mut inner = self.inner.lock().unwrap();
        inner.body = Some(Body::Text(body));
    }

    pub fn body_bytes(&self, body: Vec<u8>) {
        let mut inner = self.inner.lock().unwrap();
        inner.body = Some(Body::Binary(body));
    }

    pub fn error(&self, error: HttpError) {
        let (status_code, message) = error.to_tuple();
        let mut inner = self.inner.lock().unwrap();
        inner.status_code = status_code;
        inner.body = Some(Body::Text(message.to_string()));
    }

    pub fn send(&self, stream: &mut dyn Write) -> Result<()> {
        let inner = self.inner.lock().unwrap();
        let status_line = format!("HTTP/1.1 {} OK\r\n", inner.status_code);

        stream.write_all(status_line.as_bytes())?;

        for (key, value) in &inner.headers {
            let header_line = format!("{}: {}\r\n", key, value);
            stream.write_all(header_line.as_bytes())?;
        }

        stream.write_all(b"\r\n")?;

        if let Some(ref body) = inner.body {
            match body {
                Body::Text(ref text) => {
                    stream.write_all(text.as_bytes())?;
                }
                Body::Binary(ref binary) => {
                    stream.write_all(binary)?;
                }
            }
        }

        stream.flush()?;
        Ok(())
    }

    pub fn send_file(&self, file_path: &str) -> std::result::Result<(), HttpError> {
        let path = Path::new(file_path);
        if !path.exists() {
            let mut inner = self.inner.lock().unwrap();
            inner.status_code = 404;
            inner.body = Some(Body::Text("File not found".to_string()));
            return Err(HttpError::NotFound("File not found".to_string()));
        }

        match File::open(path) {
            Ok(mut file) => {
                let mut contents = Vec::new();
                if let Err(e) = file.read_to_end(&mut contents) {
                    return self.set_internal_server_error(format!("Failed to read file: {}", e));
                }

                let mime_type = path.extension()
                    .and_then(|ext| ext.to_str())
                    .map(get_mime_type)
                    .unwrap_or("application/octet-stream".to_string());

                self.header("Content-Type", mime_type.as_str());
                self.body_bytes(contents);
            }
            Err(e) => {
                return self.set_internal_server_error(format!("Failed to open file: {}", e));
            }
        }

        Ok(())
    }

    fn set_internal_server_error(&self, message: String) -> std::result::Result<(), HttpError> {
        let mut inner = self.inner.lock().unwrap();
        inner.status_code = 500;
        inner.body = Some(Body::Text(format!("Internal Server Error: {}", message)));
        Err(HttpError::InternalServerError(message))
    }

    pub fn get_status(&self) -> u16 {
        let inner = self.inner.lock().unwrap();
        inner.status_code
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        let inner = self.inner.lock().unwrap();
        inner.headers.get(key).cloned()
    }

    pub fn get_body(&self) -> Option<Vec<u8>> {
        let inner = self.inner.lock().unwrap();
        match &inner.body {
            Some(Body::Text(text)) => Some(text.clone().into_bytes()),
            Some(Body::Binary(binary)) => Some(binary.clone()),
            None => None,
        }
    }
}

impl Clone for Response {
    fn clone(&self) -> Self {
        Response {
            inner: Arc::clone(&self.inner),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_response_new() {
        let response = Response::new();
        assert_eq!(response.get_status(), 200);
    }

    #[test]
    fn test_response_set_status() {
        let response = Response::new();
        response.set_status(404);
        assert_eq!(response.get_status(), 404);
    }

    #[test]
    fn test_response_header() {
        let response = Response::new();
        response.header("Content-Type", "text/html");
        assert_eq!(
            response.get_header("Content-Type"),
            Some("text/html".to_string())
        );
    }

    #[test]
    fn test_response_body() {
        let response = Response::new();
        response.body("Hello, world!".to_string());
        assert_eq!(response.get_body(), Some("Hello, world!".to_string().into_bytes()));
    }

    #[test]
    fn test_response_body_bytes() {
        let response = Response::new();
        let binary_data = vec![1, 2, 3, 4, 5];
        response.body_bytes(binary_data.clone());
        assert_eq!(response.get_body(), Some(binary_data));
    }

    #[test]
    fn test_response_error() {
        let response = Response::new();
        let error = HttpError::NotFound("The requested resource was not found".to_string());
        response.error(error);
        assert_eq!(response.get_status(), 404);
        assert_eq!(
            response.get_body(),
            Some("The requested resource was not found".to_string().into_bytes())
        );
    }

    #[test]
    fn test_response_send() {
        let response = Response::new();
        let mut stream = Cursor::new(Vec::new());
        response.send(&mut stream).unwrap();
        let result = String::from_utf8(stream.into_inner()).unwrap();
        assert!(result.contains("HTTP/1.1 200 OK\r\n"));
    }

    #[test]
    fn test_response_send_file() {
        let file_path = "test_file.txt";
        let file_content = "This is a test file.";
        {
            let mut file = File::create(file_path).unwrap();
            file.write_all(file_content.as_bytes()).unwrap();
        }

        let response = Response::new();
        response.send_file(file_path).unwrap();
        assert_eq!(response.get_body(), Some(file_content.to_string().into_bytes()));
        assert_eq!(response.get_header("Content-Type"), Some("text/plain".to_string()));

        std::fs::remove_file(file_path).unwrap();
    }
}
