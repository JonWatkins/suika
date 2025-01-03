use suika_server:: {
    request::Request,
    response::Response,
    middleware::{Middleware, MiddlewareFuture, Next},
};

const WASM_BINARY: &[u8] = include_bytes!("../wasm/suika_ui_bg.wasm");
const JS_FILE: &str = include_str!("../wasm/suika_ui.js");

/// A middleware component for serving WebAssembly and JavaScript files.
///
/// This middleware serves the `suika_ui_bg.wasm` and `suika_ui.js` files when
/// the request path matches the specified URL prefix. For other paths, it
/// passes the request to the next middleware in the stack.
///
/// # Examples
///
/// ```
/// use suika_server::request::Request;
/// use suika_server::response::Response;
/// use suika_server::middleware::{Middleware, Next, MiddlewareFuture};
/// use suika_wasm::WasmFileMiddleware;
/// use std::sync::Arc;
/// use tokio::sync::Mutex;
///
/// #[derive(Clone)]
/// struct MockNextMiddleware {
///     called: Arc<Mutex<bool>>,
/// }
///
/// impl MockNextMiddleware {
///     fn new() -> Self {
///         Self {
///             called: Arc::new(Mutex::new(false)),
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
///     let mut req = Request::new("GET /static/suika_ui_bg.wasm HTTP/1.1\r\n\r\n").unwrap();
///     let mut res = Response::new(None);
///
///     let wasm_file_middleware = WasmFileMiddleware::new("/static", 3600);
///     let next_middleware = MockNextMiddleware::new();
///     let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> = vec![Arc::new(next_middleware.clone())];
///     let next = Next::new(middleware_stack.as_slice());
///
///     wasm_file_middleware.handle(&mut req, &mut res, next.clone()).await.unwrap();
///     let inner = res.get_inner().await;
///
///     assert_eq!(inner.status_code(), Some(200));
///     assert_eq!(inner.headers().get("Content-Type"), Some(&"application/wasm".to_string()));
/// }
/// ```
pub struct WasmFileMiddleware {
    url_prefix: &'static str,
    cache_duration: u64,
}

impl WasmFileMiddleware {
    /// Creates a new `WasmFileMiddleware`.
    ///
    /// # Arguments
    ///
    /// * `url_prefix` - The URL prefix for serving WebAssembly and JavaScript files.
    /// * `cache_duration` - The cache duration in seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_wasm::WasmFileMiddleware;
    ///
    /// let wasm_file_middleware = WasmFileMiddleware::new("/static", 3600);
    /// ```
    pub fn new(url_prefix: &'static str, cache_duration: u64) -> Self {
        Self {
            url_prefix,
            cache_duration,
        }
    }
}

impl Middleware for WasmFileMiddleware {
    /// Handles an incoming HTTP request by serving the `.wasm` or `.js` file if the request path matches the URL prefix.
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
    /// use suika_wasm::WasmFileMiddleware;
    /// use std::sync::Arc;
    /// use tokio::sync::Mutex;
    ///
    /// #[derive(Clone)]
    /// struct MockNextMiddleware {
    ///     called: Arc<Mutex<bool>>,
    /// }
    ///
    /// impl MockNextMiddleware {
    ///     fn new() -> Self {
    ///         Self {
    ///             called: Arc::new(Mutex::new(false)),
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
    ///     let mut req = Request::new("GET /static/suika_ui.js HTTP/1.1\r\n\r\n").unwrap();
    ///     let mut res = Response::new(None);
    ///
    ///     let wasm_file_middleware = WasmFileMiddleware::new("/static", 3600);
    ///     let next_middleware = MockNextMiddleware::new();
    ///     let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> = vec![Arc::new(next_middleware.clone())];
    ///     let next = Next::new(middleware_stack.as_slice());
    ///
    ///     wasm_file_middleware.handle(&mut req, &mut res, next.clone()).await.unwrap();
    ///     let inner = res.get_inner().await;
    ///
    ///     assert_eq!(inner.status_code(), Some(200));
    ///     assert_eq!(inner.headers().get("Content-Type"), Some(&"application/javascript".to_string()));
    /// }
    /// ```
    fn handle<'a>(
        &'a self,
        req: &'a mut Request,
        res: &'a mut Response,
        mut next: Next<'a>,
    ) -> MiddlewareFuture<'a> {
        let path = req.path().to_string();
        let url_prefix = self.url_prefix.to_string();
        let cache_duration = self.cache_duration;

        Box::pin(async move {
            if path == format!("{}/suika_ui_bg.wasm", url_prefix) {
                res.header("Content-Type", "application/wasm").await;
                res.header(
                    "Cache-Control",
                    &format!("public, max-age={}", cache_duration),
                ).await;
                res.set_status(200).await;
                res.body_bytes(WASM_BINARY.to_vec()).await;
                Ok(())
            } else if path == format!("{}/suika_ui.js", url_prefix) {
                res.header("Content-Type", "application/javascript").await;
                res.header(
                    "Cache-Control",
                    &format!("public, max-age={}", cache_duration),
                ).await;
                res.set_status(200).await;
                res.body(JS_FILE.to_string()).await;
                Ok(())
            } else {
                next.run(req, res).await
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use suika_server::request::Request;
    use suika_server::response::{Response, Body};
    use suika_server::middleware::{Middleware, Next};
    use tokio::sync::Mutex;
    use std::sync::Arc;

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
    async fn test_wasm_file_middleware_serves_wasm_file() {
        let mut req = Request::new("GET /static/suika_ui_bg.wasm HTTP/1.1\r\n\r\n").unwrap();
        let mut res = Response::new(None);

        let wasm_file_middleware = WasmFileMiddleware::new("/static", 3600);
        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> = vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        wasm_file_middleware.handle(&mut req, &mut res, next.clone()).await.unwrap();

        let inner = res.get_inner().await;
        assert_eq!(inner.status_code(), Some(200));
        assert_eq!(inner.headers().get("Content-Type"), Some(&"application/wasm".to_string()));
        assert_eq!(inner.body(), &Some(Body::Binary(WASM_BINARY.to_vec())));

        let next_called = *next_middleware.called.lock().await;
        assert!(!next_called);
    }

    #[tokio::test]
    async fn test_wasm_file_middleware_serves_js_file() {
        let mut req = Request::new("GET /static/suika_ui.js HTTP/1.1\r\n\r\n").unwrap();
        let mut res = Response::new(None);

        let wasm_file_middleware = WasmFileMiddleware::new("/static", 3600);
        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> = vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        wasm_file_middleware.handle(&mut req, &mut res, next.clone()).await.unwrap();

        let inner = res.get_inner().await;
        assert_eq!(inner.status_code(), Some(200));
        assert_eq!(inner.headers().get("Content-Type"), Some(&"application/javascript".to_string()));
        assert_eq!(inner.body(), &Some(Body::Text(JS_FILE.to_string())));

        let next_called = *next_middleware.called.lock().await;
        assert!(!next_called);
    }

    #[tokio::test]
    async fn test_wasm_file_middleware_passes_other_paths() {
        let mut req = Request::new("GET /other/path HTTP/1.1\r\n\r\n").unwrap();
        let mut res = Response::new(None);

        let wasm_file_middleware = WasmFileMiddleware::new("/static", 3600);
        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> = vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        wasm_file_middleware.handle(&mut req, &mut res, next.clone()).await.unwrap();

        let next_called = *next_middleware.called.lock().await;
        assert!(next_called);
    }
}
