use crate::error::HttpError;
use crate::middleware::{Middleware, MiddlewareFuture, Next};
use crate::request::Request;
use crate::response::Response;
use std::path::Path;
use suika_mime::get_mime_type;

async fn handle_static_file_request(
    path: String,
    cache_duration: u64,
    res: &mut Response,
) -> Result<(), HttpError> {
    if Path::new(&path).exists() {
        if let Err(e) = res.send_file(&path).await {
            res.set_status(500).await;
            res.body(format!("Internal Server Error: {}", e)).await;
            return Err(HttpError::InternalServerError(format!(
                "Internal Server Error: {}",
                e
            )));
        } else {
            let mime_type = Path::new(&path)
                .extension()
                .and_then(|ext| ext.to_str())
                .map(get_mime_type)
                .unwrap_or_else(|| "application/octet-stream".to_string());
            res.header("Content-Type", mime_type.as_str()).await;

            res.header(
                "Cache-Control",
                &format!("public, max-age={}", cache_duration),
            )
            .await;

            res.set_status(200).await;
        }
        Ok(())
    } else {
        println!("File not found: {}", path);
        Err(HttpError::NotFound("File not found".to_string()))
    }
}

/// A middleware component for serving static files.
pub struct StaticFileMiddleware {
    url_prefix: String,
    directory: String,
    cache_duration: u64,
}

impl StaticFileMiddleware {
    /// Creates a new `StaticFileMiddleware`.
    ///
    /// # Arguments
    ///
    /// * `url_prefix` - The URL prefix for serving static files.
    /// * `directory` - The directory containing static files.
    /// * `cache_duration` - The cache duration in seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::middleware::StaticFileMiddleware;
    ///
    /// let static_file_middleware = StaticFileMiddleware::new("/static", "public", 3600);
    /// ```
    pub fn new(url_prefix: &str, directory: &str, cache_duration: u64) -> Self {
        Self {
            url_prefix: url_prefix.to_string(),
            directory: directory.to_string(),
            cache_duration,
        }
    }
}

impl Middleware for StaticFileMiddleware {
    /// Handles an incoming HTTP request by serving a static file if the request path matches the URL prefix.
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
        let url_prefix = self.url_prefix.clone();
        let directory = self.directory.clone();
        let cache_duration = self.cache_duration;

        Box::pin(async move {
            let path = if let Some(stripped_path) = req.path().strip_prefix(&url_prefix) {
                format!("{}{}", directory, stripped_path)
            } else {
                return next.run(req, res).await;
            };

            match handle_static_file_request(path, cache_duration, res).await {
                Ok(_) => Ok(()),
                Err(_) => next.run(req, res).await,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::{Middleware, Next};
    use crate::request::Request;
    use crate::response::{Body, Response};
    use std::io::Write;
    use std::sync::Arc;
    use tempfile::Builder;
    use tokio::sync::Mutex;

    // Mock Next middleware
    #[derive(Clone)]
    struct MockNextMiddleware {
        called: Arc<Mutex<bool>>,
    }

    impl MockNextMiddleware {
        fn new() -> Self {
            Self {
                called: Arc::new(Mutex::new(false)),
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
    async fn test_static_file_middleware_serves_file() {
        // Create a temporary file to act as a static file
        let mut tempfile = Builder::new().suffix(".txt").tempfile().unwrap();
        tempfile.write_all(b"fake file data").unwrap();
        let file_dir = tempfile
            .path()
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let mut req = Request::new(&format!(
            "GET /static/{} HTTP/1.1\r\n\r\n",
            tempfile.path().file_name().unwrap().to_str().unwrap()
        ))
        .unwrap();
        let mut res = Response::new();

        let static_file_middleware = StaticFileMiddleware::new("/static", &file_dir, 3600);
        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> =
            vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        static_file_middleware
            .handle(&mut req, &mut res, next.clone())
            .await
            .unwrap();

        let inner = res.get_inner().await;
        assert_eq!(inner.status_code(), Some(200));
        assert_eq!(
            inner.headers().get("Content-Type"),
            Some(&"text/plain".to_string())
        );
        assert_eq!(
            inner.body(),
            &Some(Body::Binary(b"fake file data".to_vec()))
        );

        let next_called = *next_middleware.called.lock().await;
        assert!(!next_called);
    }

    #[tokio::test]
    async fn test_static_file_middleware_passes_other_paths() {
        let mut req = Request::new("GET /other/path HTTP/1.1\r\n\r\n").unwrap();
        let mut res = Response::new();

        let static_file_middleware = StaticFileMiddleware::new("/static", "some/directory", 3600);
        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> =
            vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        static_file_middleware
            .handle(&mut req, &mut res, next.clone())
            .await
            .unwrap();

        let next_called = *next_middleware.called.lock().await;
        assert!(next_called);
    }
}
