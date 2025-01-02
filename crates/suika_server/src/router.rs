use crate::error::HttpError;
use crate::middleware::{Middleware, MiddlewareFuture, Next};
use crate::request::Request;
use crate::response::Response;
use regex::Regex;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

/// Represents a route in the router.
pub struct Route {
    pub method: Option<String>,
    pub pattern: Regex,
    pub handler: Arc<
        dyn for<'a> Fn(&'a mut Request, &'a mut Response) -> MiddlewareFuture<'a> + Send + Sync,
    >,
}

/// A router for handling HTTP requests and routing them to appropriate handlers.
///
/// The `Router` can handle routes with or without parameters, and it supports mounting sub-routers.
///
/// # Examples
///
/// ```
/// use suika_server::request::Request;
/// use suika_server::response::{Response, Body};
/// use suika_server::middleware::{Middleware, Next, MiddlewareFuture};
/// use suika_server::router::Router;
/// use regex::Regex;
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
///     let mut router = Router::new("/api");
///
///     router.add_route(Some("GET"), "/test", |req, res| {
///         Box::pin(async move {
///             res.set_status(200).await;
///             res.body("Test route".to_string()).await;
///             Ok(())
///         })
///     });
///
///     let mut req = Request::new("GET /api/test HTTP/1.1\r\n\r\n").unwrap();
///     let mut res = Response::new();
///
///     let next_middleware = MockNextMiddleware::new();
///     let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> = vec![Arc::new(next_middleware.clone())];
///     let next = Next::new(middleware_stack.as_slice());
///
///     router.handle(&mut req, &mut res, next.clone()).await.unwrap();
///
///     let inner = res.get_inner().await;
///     assert_eq!(inner.status_code(), Some(200));
///     assert_eq!(inner.body(), &Some(Body::Text("Test route".to_string())));
///
///     let next_called = *next_middleware.called.lock().await;
///     assert!(!next_called);
/// }
/// ```
pub struct Router {
    pub base_path: String,
    pub routes: Vec<Route>,
    pub sub_routers: Vec<Router>,
}

impl Router {
    /// Creates a new `Router` with the specified base path.
    ///
    /// # Arguments
    ///
    /// * `base_path` - The base path for the router.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::router::Router;
    ///
    /// let router = Router::new("/api");
    /// ```
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
            routes: Vec::new(),
            sub_routers: Vec::new(),
        }
    }

    /// Adds a route to the router.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method for the route (e.g., "GET", "POST").
    /// * `pattern` - The URL pattern for the route, which can include named parameters.
    /// * `handler` - The handler function for the route.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use suika_server::response::Response;
    /// use suika_server::middleware::MiddlewareFuture;
    /// use suika_server::router::Router;
    /// use std::sync::Arc;
    ///
    /// let mut router = Router::new("/api");
    ///
    /// router.add_route(Some("GET"), "/test", |req, res| {
    ///     Box::pin(async move {
    ///         res.set_status(200).await;
    ///         res.body("Test route".to_string()).await;
    ///         Ok(())
    ///     })
    /// });
    /// ```
    pub fn add_route<F>(&mut self, method: Option<&str>, pattern: &str, handler: F)
    where
        F: for<'a> Fn(&'a mut Request, &'a mut Response) -> MiddlewareFuture<'a>
            + Send
            + Sync
            + 'static,
    {
        let full_pattern = format!("{}{}", self.base_path.trim_end_matches('/'), pattern);
        let rgx = Regex::new(&full_pattern).expect("Invalid regex pattern");

        self.routes.push(Route {
            method: method.map(|m| m.to_string()),
            pattern: rgx,
            handler: Arc::new(handler),
        });
    }

    /// Mounts a sub-router onto this router.
    ///
    /// # Arguments
    ///
    /// * `sub_router` - The sub-router to mount.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::router::Router;
    ///
    /// let mut router = Router::new("/api");
    /// let sub_router = Router::new("/sub");
    ///
    /// router.mount(sub_router);
    /// ```
    pub fn mount(&mut self, mut sub_router: Router) {
        let combined = format!(
            "{}{}",
            self.base_path.trim_end_matches('/'),
            sub_router.base_path
        );
        sub_router.base_path = combined;
        self.sub_routers.push(sub_router);
    }

    /// Handles an incoming HTTP request by matching it to a route.
    ///
    /// This method is called internally by the `Router`'s `Middleware` implementation.
    ///
    /// # Arguments
    ///
    /// * `req` - A mutable reference to the incoming request.
    /// * `res` - A mutable reference to the response to be sent.
    ///
    /// # Returns
    ///
    /// A future that resolves to `Result<bool, HttpError>`, indicating whether a route was matched.
    fn handle_internal<'a>(
        &'a self,
        req: &'a mut Request,
        res: &'a mut Response,
    ) -> Pin<Box<dyn futures::Future<Output = Result<bool, HttpError>> + Send + 'a>> {
        Box::pin(async move {
            for route in &self.routes {
                if let Some(ref route_method) = route.method {
                    if route_method.to_uppercase() != req.method().to_uppercase() {
                        continue;
                    }
                }
                if let Some(caps) = route.pattern.captures(req.path()) {
                    let mut params = HashMap::new();
                    for name in route.pattern.capture_names().flatten() {
                        if let Some(value) = caps.name(name) {
                            params.insert(name.to_string(), value.as_str().to_string());
                        }
                    }

                    req.set_params(params);

                    if let Err(e) = (route.handler)(req, res).await {
                        res.error(e).await;
                        return Ok(true);
                    }
                    return Ok(true);
                }
            }

            for subr in &self.sub_routers {
                if subr.handle_internal(req, res).await? {
                    return Ok(true);
                }
            }
            Ok(false)
        })
    }
}

impl Middleware for Router {
    /// Handles an incoming HTTP request by routing it to the appropriate handler.
    ///
    /// # Arguments
    ///
    /// * `req` - A mutable reference to the incoming request.
    /// * `res` - A mutable reference to the response.
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
    /// use suika_server::response::{Response, Body};
    /// use suika_server::middleware::{Middleware, Next, MiddlewareFuture};
    /// use suika_server::router::Router;
    /// use regex::Regex;
    /// use std::sync::Arc;
    /// use tokio::sync::Mutex;
    /// 
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
    ///     let mut router = Router::new("/api");
    ///
    ///     router.add_route(Some("GET"), "/test", |req, res| {
    ///         Box::pin(async move {
    ///             res.set_status(200).await;
    ///             res.body("Test route".to_string()).await;
    ///             Ok(())
    ///         })
    ///     });
    ///
    ///     let mut req = Request::new("GET /api/test HTTP/1.1\r\n\r\n").unwrap();
    ///     let mut res = Response::new();
    ///
    ///     let next_middleware = MockNextMiddleware::new();
    ///     let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> = vec![Arc::new(next_middleware.clone())];
    ///     let next = Next::new(middleware_stack.as_slice());
    ///
    ///     router.handle(&mut req, &mut res, next.clone()).await.unwrap();
    ///
    ///     let inner = res.get_inner().await;
    ///     assert_eq!(inner.status_code(), Some(200));
    ///     assert_eq!(inner.body(), &Some(Body::Text("Test route".to_string())));
    ///
    ///     let next_called = *next_middleware.called.lock().await;
    ///     assert!(!next_called);
    /// }
    /// ```
    fn handle<'a>(
        &'a self,
        req: &'a mut Request,
        res: &'a mut Response,
        mut next: Next<'a>,
    ) -> MiddlewareFuture<'a> {
        Box::pin(async move {
            let matched_route = self.handle_internal(req, res).await;
            if let Err(e) = matched_route {
                res.error(e).await;
            } else if !matched_route.unwrap_or(false) {
                next.run(req, res).await?;
            }
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::{Middleware, Next};
    use crate::request::Request;
    use crate::response::{Body, Response};
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
    async fn test_router_handles_route() {
        let mut router = Router::new("/api");

        router.add_route(Some("GET"), "/test", |_req, res| {
            Box::pin(async move {
                res.set_status(200).await;
                res.body("Test route".to_string()).await;
                Ok(())
            })
        });

        let mut req = Request::new("GET /api/test HTTP/1.1\r\n\r\n").unwrap();
        let mut res = Response::new();

        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> =
            vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        router
            .handle(&mut req, &mut res, next.clone())
            .await
            .unwrap();

        let inner = res.get_inner().await;
        assert_eq!(inner.status_code(), Some(200));
        assert_eq!(inner.body(), &Some(Body::Text("Test route".to_string())));

        let next_called = *next_middleware.called.lock().await;
        assert!(!next_called);
    }

    #[tokio::test]
    async fn test_router_handles_route_with_params() {
        let mut router = Router::new("/api");

        router.add_route(Some("GET"), "/test/(?P<id>\\d+)", |req, res| {
            Box::pin(async move {
                let id = req.param("id").expect("Expected id parameter");
                res.set_status(200).await;
                res.body(format!("Test route with id: {}", id)).await;
                Ok(())
            })
        });

        let mut req = Request::new("GET /api/test/123 HTTP/1.1\r\n\r\n").unwrap();
        let mut res = Response::new();

        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> =
            vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        router
            .handle(&mut req, &mut res, next.clone())
            .await
            .unwrap();

        let inner = res.get_inner().await;
        assert_eq!(inner.status_code(), Some(200));
        assert_eq!(
            inner.body(),
            &Some(Body::Text("Test route with id: 123".to_string()))
        );

        let next_called = *next_middleware.called.lock().await;
        assert!(!next_called);
    }

    #[tokio::test]
    async fn test_router_handles_sub_router() {
        let mut router = Router::new("/api");
        let mut sub_router = Router::new("/sub");

        sub_router.add_route(Some("GET"), "/test", |_req, res| {
            Box::pin(async move {
                res.set_status(200).await;
                res.body("Sub router test route".to_string()).await;
                Ok(())
            })
        });

        router.mount(sub_router);

        let mut req = Request::new("GET /api/sub/test HTTP/1.1\r\n\r\n").unwrap();
        let mut res = Response::new();

        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> =
            vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        router
            .handle(&mut req, &mut res, next.clone())
            .await
            .unwrap();

        let inner = res.get_inner().await;
        assert_eq!(inner.status_code(), Some(200));
        assert_eq!(
            inner.body(),
            &Some(Body::Text("Sub router test route".to_string()))
        );

        let next_called = *next_middleware.called.lock().await;
        assert!(!next_called);
    }

    #[tokio::test]
    async fn test_router_passes_to_next_middleware() {
        let mut router = Router::new("/api");

        router.add_route(Some("GET"), "/test", |_req, res| {
            Box::pin(async move {
                res.set_status(200).await;
                res.body("Test route".to_string()).await;
                Ok(())
            })
        });

        let mut req = Request::new("GET /other/path HTTP/1.1\r\n\r\n").unwrap();
        let mut res = Response::new();

        let next_middleware = MockNextMiddleware::new();
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> =
            vec![Arc::new(next_middleware.clone())];
        let next = Next::new(middleware_stack.as_slice());

        router
            .handle(&mut req, &mut res, next.clone())
            .await
            .unwrap();

        let next_called = *next_middleware.called.lock().await;
        assert!(next_called);
    }
}
