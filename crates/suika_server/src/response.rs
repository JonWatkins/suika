use crate::error::HttpError;
use std::collections::HashMap;
use std::io::Result as IoResult;
use std::path::Path;
use std::sync::Arc;
use suika_json::JsonValue;
use suika_mime::get_mime_type_from_path;
use suika_templates::TemplateEngine;
use suika_templates::context::Context;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

/// Represents an HTTP response.
#[derive(Debug)]
pub struct Response {
    inner: Arc<Mutex<ResponseInner>>,
}

/// Represents the inner state of the HTTP response.
#[derive(Debug, Clone)]
pub struct ResponseInner {
    status_code: Option<u16>,
    headers: HashMap<String, String>,
    body: Option<Body>,
    template_engine: Option<Arc<TemplateEngine>>, // Add template engine field
}

impl ResponseInner {
    /// Returns the status code of the response.
    pub fn status_code(&self) -> Option<u16> {
        self.status_code
    }

    /// Returns the headers of the response.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Returns the body of the response.
    pub fn body(&self) -> &Option<Body> {
        &self.body
    }
}

/// Represents the body of the HTTP response.
#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    Text(String),
    Binary(Vec<u8>),
}

impl Response {
    /// Creates a new `Response` with default values.
    pub fn new(template_engine: Option<Arc<TemplateEngine>>) -> Self {
        Response {
            inner: Arc::new(Mutex::new(ResponseInner {
                status_code: None,
                headers: HashMap::new(),
                body: None,
                template_engine,
            })),
        }
    }

    /// Sets the status code of the response.
    pub async fn set_status(&self, code: u16) {
        let mut inner = self.inner.lock().await;
        inner.status_code = Some(code);
    }

    /// Gets the status code of the response.
    pub async fn status(&self) -> Option<u16> {
        let inner = self.inner.lock().await;
        inner.status_code
    }

    /// Adds a header to the response.
    pub async fn header(&self, key: &str, value: &str) {
        let mut inner = self.inner.lock().await;
        inner.headers.insert(key.to_string(), value.to_string());
    }

    /// Sets the body of the response to a text string.
    pub async fn body(&self, body: String) {
        let mut inner = self.inner.lock().await;
        inner.body = Some(Body::Text(body));
    }

    /// Sets the body of the response to a JSON value.
    pub async fn body_json(&self, json_value: JsonValue) {
        let mut inner = self.inner.lock().await;
        inner
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        inner.body = Some(Body::Text(json_value.to_string()));
    }

    /// Sets the body of the response to binary data.
    pub async fn body_bytes(&self, body: Vec<u8>) {
        let mut inner = self.inner.lock().await;
        inner.body = Some(Body::Binary(body));
    }

    /// Sets the response to an HTTP error.
    pub async fn error(&self, http_error: HttpError) {
        let mut inner = self.inner.lock().await;
        let (status_code, message) = http_error.to_tuple();
        inner.status_code = Some(status_code);
        inner.body = Some(Body::Text(message.to_string()));
    }

    /// Sends the response over a stream.
    pub async fn send(&self, stream: &mut (impl AsyncWriteExt + Unpin)) -> IoResult<()> {
        let inner = self.inner.lock().await;
        let status_code = inner.status_code.unwrap_or(200);
        let status_text = match status_code {
            200 => "OK",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown Status",
        };
        let status_line = format!("HTTP/1.1 {} {}\r\n", status_code, status_text);

        stream.write_all(status_line.as_bytes()).await?;

        for (key, value) in &inner.headers {
            let header_line = format!("{}: {}\r\n", key, value);
            stream.write_all(header_line.as_bytes()).await?;
        }

        stream.write_all(b"\r\n").await?;

        if let Some(ref body) = inner.body {
            match body {
                Body::Text(ref text) => {
                    stream.write_all(text.as_bytes()).await?;
                }
                Body::Binary(ref binary) => {
                    stream.write_all(binary).await?;
                }
            }
        }

        stream.flush().await?;
        Ok(())
    }

    /// Sends a file as the response body.
    pub async fn send_file(&self, file_path: &str) -> Result<(), HttpError> {
        let path = Path::new(file_path);

        if !path.exists() {
            return Err(HttpError::NotFound("File not found".to_string()));
        }

        let mut file = File::open(path)
            .await
            .map_err(|e| HttpError::InternalServerError(format!("Failed to open file: {}", e)))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .await
            .map_err(|e| HttpError::InternalServerError(format!("Failed to read file: {}", e)))?;

        let mime_type = get_mime_type_from_path(file_path);
        self.header("Content-Type", mime_type.as_ref()).await;
        self.header("Content-Length", &buffer.len().to_string())
            .await;

        self.set_status(200).await;
        self.body_bytes(buffer).await;

        Ok(())
    }

    /// Renders a template using the template engine and sets it as the response body.
    pub async fn render_template(
        &self,
        template_name: &str,
        context: &Context,
    ) -> Result<(), HttpError> {
        let inner = self.inner.lock().await;
        if let Some(template_engine) = &inner.template_engine {
            let rendered = template_engine
                .render(template_name, &context)
                .map_err(|e| {
                    HttpError::InternalServerError(format!("Failed to render template: {}", e))
                })?;
            drop(inner);
            self.body(rendered).await;
            Ok(())
        } else {
            Err(HttpError::InternalServerError(
                "Template engine not available".to_string(),
            ))
        }
    }

    /// Returns the inner state of the response.
    pub async fn get_inner(&self) -> ResponseInner {
        self.inner.lock().await.clone()
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
    use crate::error::HttpError;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::task::{Context as STDContext, Poll};
    use suika_templates::template_engine::TemplateEngine;
    use tokio::io::{AsyncWrite, AsyncWriteExt};
    use tokio::sync::Mutex;

    struct MockStream {
        data: Arc<Mutex<Vec<u8>>>,
    }

    impl MockStream {
        fn new() -> Self {
            MockStream {
                data: Arc::new(Mutex::new(Vec::new())),
            }
        }

        async fn get_data(&self) -> Vec<u8> {
            let data = self.data.lock().await;
            data.clone()
        }
    }

    impl AsyncWrite for MockStream {
        fn poll_write(
            self: Pin<&mut Self>,
            _: &mut STDContext<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            let mut data = futures::executor::block_on(self.data.lock());
            data.extend_from_slice(buf);
            Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(self: Pin<&mut Self>, _: &mut STDContext<'_>) -> Poll<std::io::Result<()>> {
            Poll::Ready(Ok(()))
        }

        fn poll_shutdown(self: Pin<&mut Self>, _: &mut STDContext<'_>) -> Poll<std::io::Result<()>> {
            Poll::Ready(Ok(()))
        }
    }

    #[tokio::test]
    async fn test_set_status() {
        let response = Response::new(None);
        response.set_status(404).await;
        let inner = response.inner.lock().await;
        assert_eq!(inner.status_code, Some(404));
    }

    #[tokio::test]
    async fn test_status() {
        let response = Response::new(None);
        assert_eq!(response.status().await, None);
        response.set_status(200).await;
        assert_eq!(response.status().await, Some(200));
    }

    #[tokio::test]
    async fn test_header() {
        let response = Response::new(None);
        response.header("Content-Type", "text/plain").await;
        let inner = response.inner.lock().await;
        assert_eq!(
            inner.headers.get("Content-Type"),
            Some(&"text/plain".to_string())
        );
    }

    #[tokio::test]
    async fn test_body() {
        let response = Response::new(None);
        response.body("Hello, world!".to_string()).await;
        let inner = response.inner.lock().await;
        if let Some(Body::Text(ref text)) = inner.body {
            assert_eq!(text, "Hello, world!");
        } else {
            panic!("Expected body to be Some(Body::Text)");
        }
    }

    #[tokio::test]
    async fn test_body_bytes() {
        let response = Response::new(None);
        response.body_bytes(vec![1, 2, 3, 4]).await;
        let inner = response.inner.lock().await;
        if let Some(Body::Binary(ref bytes)) = inner.body {
            assert_eq!(bytes, &vec![1, 2, 3, 4]);
        } else {
            panic!("Expected body to be Some(Body::Binary)");
        }
    }

    #[tokio::test]
    async fn test_error() {
        let response = Response::new(None);
        response
            .error(HttpError::NotFound("Resource not found".to_string()))
            .await;
        let inner = response.inner.lock().await;
        assert_eq!(inner.status_code, Some(404));
        if let Some(Body::Text(ref text)) = inner.body {
            assert_eq!(text, "Resource not found");
        } else {
            panic!("Expected body to be Some(Body::Text)");
        }
    }

    #[tokio::test]
    async fn test_send() {
        let response = Response::new(None);
        response.set_status(200).await;
        response.header("Content-Type", "text/plain").await;
        response.body("Hello, world!".to_string()).await;

        let mut mock_stream = MockStream::new();
        response.send(&mut mock_stream).await.unwrap();

        let data = mock_stream.get_data().await;
        let response_string = String::from_utf8(data).unwrap();
        assert!(response_string.contains("HTTP/1.1 200 OK"));
        assert!(response_string.contains("Content-Type: text/plain"));
        assert!(response_string.contains("Hello, world!"));
    }

    #[tokio::test]
    async fn test_send_file() {
        // Create a temporary file for testing
        let file_path = "test_file.txt";
        let mut file = File::create(file_path).await.unwrap();
        file.write_all(b"File content").await.unwrap();

        let response = Response::new(None);
        response.send_file(file_path).await.unwrap();

        let inner = response.inner.lock().await;
        assert_eq!(
            inner.headers.get("Content-Type"),
            Some(&"text/plain".to_string())
        );
        assert_eq!(inner.headers.get("Content-Length"), Some(&"12".to_string()));
        if let Some(Body::Binary(ref bytes)) = inner.body {
            assert_eq!(bytes, &b"File content"[..]);
        } else {
            panic!("Expected body to be Some(Body::Binary)");
        }

        // Clean up the temporary file
        tokio::fs::remove_file(file_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_render_template() {
        let mut template_engine = TemplateEngine::new();
        template_engine.add_template(
            "hello.html",
            "<html><body>Hello, <%= name %>!</body></html>",
        );

        let template_engine = Arc::new(template_engine);
        let response = Response::new(Some(template_engine.clone()));
        let mut context = Context::new();

        context.insert("name", "World");

        response
            .render_template("hello.html", &context)
            .await
            .unwrap();
        let inner = response.inner.lock().await;

        if let Some(Body::Text(ref text)) = inner.body {
            assert_eq!(text, "<html><body>Hello, World!</body></html>");
        } else {
            panic!("Expected body to be Some(Body::Text)");
        }
    }

    #[tokio::test]
    async fn test_body_json() {
        let response = Response::new(None);
        let json_value = JsonValue::Object(vec![(
            "key".to_string(),
            JsonValue::String("value".to_string()),
        )]);
        response.body_json(json_value.clone()).await;

        let inner = response.inner.lock().await;
        assert_eq!(
            inner.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        if let Some(Body::Text(ref text)) = inner.body {
            assert_eq!(text, &json_value.to_string());
        } else {
            panic!("Expected body to be Some(Body::Text)");
        }
    }
}
