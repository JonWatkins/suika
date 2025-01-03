use crate::middleware::{Middleware, MiddlewareFuture, Next};
use crate::request::Request;
use crate::response::Response;

/// A middleware component for handling CORS (Cross-Origin Resource Sharing).
pub struct CorsMiddleware;

impl Middleware for CorsMiddleware {
    /// Handles an incoming HTTP request by adding CORS headers.
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
        Box::pin(async move {
            res.header("Access-Control-Allow-Origin", "*").await;
            res.header(
                "Access-Control-Allow-Methods",
                "GET, POST, PUT, DELETE, OPTIONS",
            )
            .await;
            res.header(
                "Access-Control-Allow-Headers",
                "Content-Type, Authorization",
            )
            .await;

            if req.method() == "OPTIONS" {
                res.set_status(204).await;
                return Ok(());
            }

            next.run(req, res).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::{Middleware, Next};
    use crate::request::Request;
    use crate::response::Response;
    use std::sync::Arc;
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
    async fn test_cors_middleware_headers() {
        let mut req = Request::new("GET / HTTP/1.1\r\n\r\n").unwrap();
        let mut res = Response::new(None);

        let cors_middleware = CorsMiddleware;
        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> =
            vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        cors_middleware
            .handle(&mut req, &mut res, next.clone())
            .await
            .unwrap();

        let inner = res.get_inner().await;
        assert_eq!(
            inner.headers().get("Access-Control-Allow-Origin"),
            Some(&"*".to_string())
        );
        assert_eq!(
            inner.headers().get("Access-Control-Allow-Methods"),
            Some(&"GET, POST, PUT, DELETE, OPTIONS".to_string())
        );
        assert_eq!(
            inner.headers().get("Access-Control-Allow-Headers"),
            Some(&"Content-Type, Authorization".to_string())
        );

        let next_called = *next_middleware.called.lock().await;
        assert!(next_called);
    }

    #[tokio::test]
    async fn test_cors_middleware_options_request() {
        let mut req = Request::new("OPTIONS / HTTP/1.1\r\n\r\n").unwrap();
        let mut res = Response::new(None);

        let cors_middleware = CorsMiddleware;
        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> =
            vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        cors_middleware
            .handle(&mut req, &mut res, next.clone())
            .await
            .unwrap();

        let inner = res.get_inner().await;
        assert_eq!(
            inner.headers().get("Access-Control-Allow-Origin"),
            Some(&"*".to_string())
        );
        assert_eq!(
            inner.headers().get("Access-Control-Allow-Methods"),
            Some(&"GET, POST, PUT, DELETE, OPTIONS".to_string())
        );
        assert_eq!(
            inner.headers().get("Access-Control-Allow-Headers"),
            Some(&"Content-Type, Authorization".to_string())
        );
        assert_eq!(inner.status_code(), Some(204));

        let next_called = *next_middleware.called.lock().await;
        assert!(!next_called);
    }
}
