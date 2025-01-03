use crate::error::HttpError;
use crate::middleware::{Middleware, MiddlewareFuture, Next};
use crate::request::Request;
use crate::response::Response;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use suika_mime::get_mime_type;

fn map_io_error(err: std::io::Error) -> HttpError {
    HttpError::InternalServerError(format!("IO error: {}", err))
}

async fn handle_favicon_request(path: PathBuf, res: &mut Response) -> Result<(), HttpError> {
    let absolute_path = path.canonicalize().map_err(|e| {
        println!("Failed to get absolute path: {:?}", e);
        HttpError::NotFound("Favicon not found".to_string())
    })?;

    if !absolute_path.exists() {
        println!("File does not exist: {:?}", absolute_path);
        res.set_status(404).await;
        res.body("Favicon not found".to_string()).await;
        return Err(HttpError::NotFound("Favicon not found".to_string()));
    }

    let mut file = File::open(&absolute_path)
        .map_err(|_| HttpError::NotFound("Favicon not found".to_string()))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(map_io_error)?;

    res.body_bytes(buffer).await;

    let mime_type = absolute_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(get_mime_type)
        .unwrap_or_else(|| "application/octet-stream".to_string());

    res.set_status(200).await;
    res.header("Content-Type", mime_type.as_str()).await;

    Ok(())
}

/// A middleware component for serving a favicon.
pub struct FaviconMiddleware {
    favicon_path: String,
}

impl FaviconMiddleware {
    /// Creates a new `FaviconMiddleware`.
    ///
    /// # Arguments
    ///
    /// * `favicon_path` - The path to the favicon file.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::middleware::FaviconMiddleware;
    ///
    /// let favicon_middleware = FaviconMiddleware::new("path/to/favicon.ico");
    /// ```
    pub fn new(favicon_path: &str) -> Self {
        Self {
            favicon_path: favicon_path.to_string(),
        }
    }
}

impl Middleware for FaviconMiddleware {
    /// Handles an incoming HTTP request by serving the favicon if the request path is `/favicon.ico`.
    ///
    /// # Arguments
    ///
    /// * `req` - A mutable reference to the incoming request.
    /// * `res` - A mutable reference to the response to be sent.
    /// * `next` - The next middleware in the stack.
    ///
    /// # Returns
    ///
    /// A future that resolves to a `Result<(), HttpError>`.
    fn handle<'a>(
        &'a self,
        req: &'a mut Request,
        res: &'a mut Response,
        mut next: Next<'a>,
    ) -> MiddlewareFuture<'a> {
        let favicon_path = self.favicon_path.clone();
        Box::pin(async move {
            if req.path() == "/favicon.ico" {
                return handle_favicon_request(PathBuf::from(favicon_path), res).await;
            }
            next.run(req, res).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::{Middleware, Next};
    use crate::response::{Body, Response};
    use std::collections::HashMap;
    use std::io::Write;
    use std::sync::{Arc, Mutex};
    use tempfile::Builder;
    use tokio::sync::Mutex as TokioMutex;

    // Mock Next middleware
    #[derive(Clone)]
    struct MockNextMiddleware {
        called: Arc<TokioMutex<bool>>,
    }

    impl MockNextMiddleware {
        fn new() -> Self {
            Self {
                called: Arc::new(TokioMutex::new(false)),
            }
        }
    }

    impl Middleware for MockNextMiddleware {
        fn handle<'a>(
            &'a self,
            _req: &'a mut Request,
            _res: &'a mut Response,
            _next: Next<'a>,
        ) -> MiddlewareFuture<'a> {
            let called = Arc::clone(&self.called);
            Box::pin(async move {
                let mut called_lock = called.lock().await;
                *called_lock = true;
                Ok(())
            })
        }
    }

    #[tokio::test]
    async fn test_favicon_middleware_serves_favicon() {
        // Create a temporary file with an .ico extension to act as the favicon
        let mut tempfile = Builder::new().suffix(".ico").tempfile().unwrap();
        tempfile.write_all(b"fake favicon data").unwrap();
        let favicon_path = tempfile.path().to_str().unwrap().to_string();

        let mut req = Request::new(
            "GET /favicon.ico HTTP/1.1\r\n\r\n",
            Arc::new(Mutex::new(HashMap::new())),
        )
        .unwrap();
        let mut res = Response::new(None);

        let favicon_middleware = FaviconMiddleware::new(&favicon_path);
        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> =
            vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        favicon_middleware
            .handle(&mut req, &mut res, next.clone())
            .await
            .unwrap();

        let inner = res.get_inner().await;
        assert_eq!(inner.status_code(), Some(200));
        assert_eq!(
            inner.headers().get("Content-Type"),
            Some(&"image/x-icon".to_string())
        );
        assert_eq!(
            inner.body(),
            &Some(Body::Binary(b"fake favicon data".to_vec()))
        );

        let next_called = *next_middleware.called.lock().await;
        assert!(!next_called);
    }

    #[tokio::test]
    async fn test_favicon_middleware_passes_other_paths() {
        let mut req = Request::new(
            "GET /other/path HTTP/1.1\r\n\r\n",
            Arc::new(Mutex::new(HashMap::new())),
        )
        .unwrap();
        let mut res = Response::new(None);

        let favicon_middleware = FaviconMiddleware::new("path/to/favicon.ico");
        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> =
            vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        favicon_middleware
            .handle(&mut req, &mut res, next.clone())
            .await
            .unwrap();

        let next_called = *next_middleware.called.lock().await;
        assert!(next_called);
    }
}
