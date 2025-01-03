use crate::middleware::{Middleware, MiddlewareFuture, Next};
use crate::request::Request;
use crate::response::Response;

/// A middleware component for logging HTTP requests.
///
/// This middleware logs the HTTP method and path of incoming requests.
/// It then passes the request to the next middleware in the stack.
///
/// # Examples
///
/// ```
/// use suika_server::request::Request;
/// use suika_server::response::Response;
/// use suika_server::middleware::{Middleware, Next, MiddlewareFuture};
/// use suika_server::LoggerMiddleware;
/// use std::collections::HashMap;
/// use std::sync::{Arc, Mutex};
/// use tokio::sync::Mutex as TokioMutex;
///
/// #[derive(Clone)]
/// struct MockNextMiddleware {
///     called: Arc<TokioMutex<bool>>,
/// }
///
/// impl MockNextMiddleware {
///     fn new() -> Self {
///         Self {
///             called: Arc::new(TokioMutex::new(false)),
///         }
///     }
/// }
///
/// impl Middleware for MockNextMiddleware {
///     fn handle<'a>(
///         &'a self,
///         _req: &'a mut Request,
///         _res: &'a mut Response,
///         _next: Next<'a>,
///     ) -> MiddlewareFuture<'a> {
///         let called = Arc::clone(&self.called);
///         Box::pin(async move {
///             let mut called_lock = called.lock().await;
///             *called_lock = true;
///             Ok(())
///         })
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let mut req = Request::new(
///         "GET /test HTTP/1.1\r\n\r\n",
///         Arc::new(Mutex::new(HashMap::new())),
///     ).unwrap();
/// 
///     let mut res = Response::new(None);
///
///     let logger_middleware = LoggerMiddleware;
///     let next_middleware = MockNextMiddleware::new();
///     let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> = vec![Arc::new(next_middleware.clone())];
///     let next = Next::new(middleware_stack.as_slice());
///
///     logger_middleware.handle(&mut req, &mut res, next.clone()).await.unwrap();
///
///     let next_called = *next_middleware.called.lock().await;
///     assert!(next_called);
/// }
/// ```
pub struct LoggerMiddleware;

impl Middleware for LoggerMiddleware {
    /// Handles an incoming HTTP request by logging the method and path.
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
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use suika_server::response::Response;
    /// use suika_server::middleware::{Middleware, Next, MiddlewareFuture};
    /// use suika_server::LoggerMiddleware;
    /// use std::collections::HashMap;
    /// use std::sync::{Arc, Mutex};
    /// use tokio::sync::Mutex as TokioMutex;
    ///
    /// #[derive(Clone)]
    /// struct MockNextMiddleware {
    ///     called: Arc<TokioMutex<bool>>,
    /// }
    ///
    /// impl MockNextMiddleware {
    ///     fn new() -> Self {
    ///         Self {
    ///             called: Arc::new(TokioMutex::new(false)),
    ///         }
    ///     }
    /// }
    ///
    /// impl Middleware for MockNextMiddleware {
    ///     fn handle<'a>(
    ///         &'a self,
    ///         _req: &'a mut Request,
    ///         _res: &'a mut Response,
    ///         _next: Next<'a>,
    ///     ) -> MiddlewareFuture<'a> {
    ///         let called = Arc::clone(&self.called);
    ///         Box::pin(async move {
    ///             let mut called_lock = called.lock().await;
    ///             *called_lock = true;
    ///             Ok(())
    ///         })
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut req = Request::new(
    ///         "GET /test HTTP/1.1\r\n\r\n",
    ///         Arc::new(Mutex::new(HashMap::new())),
    ///     ).unwrap();
    /// 
    ///     let mut res = Response::new(None);
    ///
    ///     let logger_middleware = LoggerMiddleware;
    ///     let next_middleware = MockNextMiddleware::new();
    ///     let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> = vec![Arc::new(next_middleware.clone())];
    ///     let next = Next::new(middleware_stack.as_slice());
    ///
    ///     logger_middleware.handle(&mut req, &mut res, next.clone()).await.unwrap();
    ///
    ///     let next_called = *next_middleware.called.lock().await;
    ///     assert!(next_called);
    /// }
    /// ```
    fn handle<'a>(
        &'a self,
        req: &'a mut Request,
        res: &'a mut Response,
        mut next: Next<'a>,
    ) -> MiddlewareFuture<'a> {
        Box::pin(async move {
            println!("Logger => Method: {}, Path: {}", req.method(), req.path());
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
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
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
    async fn test_logger_middleware_logs_request() {
        let mut req = Request::new(
            "GET /test HTTP/1.1\r\n\r\n",
            Arc::new(Mutex::new(HashMap::new())),
        )
        .unwrap();
    
        let mut res = Response::new(None);

        let logger_middleware = LoggerMiddleware;
        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> =
            vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        logger_middleware
            .handle(&mut req, &mut res, next.clone())
            .await
            .unwrap();

        let next_called = *next_middleware.called.lock().await;
        assert!(next_called);
    }

    #[tokio::test]
    async fn test_logger_middleware_passes_request() {
        let mut req = Request::new(
            "GET /test HTTP/1.1\r\n\r\n",
            Arc::new(Mutex::new(HashMap::new())),
        )
        .unwrap();

        let mut res = Response::new(None);

        let logger_middleware = LoggerMiddleware;
        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> =
            vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        logger_middleware
            .handle(&mut req, &mut res, next.clone())
            .await
            .unwrap();

        let next_called = *next_middleware.called.lock().await;
        assert!(next_called);
    }
}
