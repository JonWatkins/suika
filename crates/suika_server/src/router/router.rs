use super::route::Route;
use crate::http::request::Request;
use crate::http::response::Response;
use crate::HttpError;
use crate::middleware::NextMiddleware;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct Router {
    routes: Vec<Route>,
    nested_routers: Vec<(String, Arc<Router>)>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Vec::new(),
            nested_routers: Vec::new(),
        }
    }

    fn add_route<F, Fut>(&mut self, method: &str, path: &str, handler: F)
    where
        F: Fn(Arc<Request>, Arc<Response>, Arc<NextMiddleware>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), HttpError>> + Send + 'static,
    {
        self.routes.push(Route::new(method, path, handler));
    }

    pub fn get<F, Fut>(&mut self, path: &str, handler: F)
    where
        F: Fn(Arc<Request>, Arc<Response>, Arc<NextMiddleware>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), HttpError>> + Send + 'static,
    {
        self.add_route("GET", path, handler);
    }

    pub fn post<F, Fut>(&mut self, path: &str, handler: F)
    where
        F: Fn(Arc<Request>, Arc<Response>, Arc<NextMiddleware>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), HttpError>> + Send + 'static,
    {
        self.add_route("POST", path, handler);
    }

    pub fn put<F, Fut>(&mut self, path: &str, handler: F)
    where
        F: Fn(Arc<Request>, Arc<Response>, Arc<NextMiddleware>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), HttpError>> + Send + 'static,
    {
        self.add_route("PUT", path, handler);
    }

    pub fn delete<F, Fut>(&mut self, path: &str, handler: F)
    where
        F: Fn(Arc<Request>, Arc<Response>, Arc<NextMiddleware>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), HttpError>> + Send + 'static,
    {
        self.add_route("DELETE", path, handler);
    }

    pub fn patch<F, Fut>(&mut self, path: &str, handler: F)
    where
        F: Fn(Arc<Request>, Arc<Response>, Arc<NextMiddleware>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), HttpError>> + Send + 'static,
    {
        self.add_route("PATCH", path, handler);
    }

    pub fn use_router(&mut self, path: &str, router: Router) {
        self.nested_routers
            .push((path.to_string(), Arc::new(router)));
    }

    fn route(
        &self,
        req: Arc<Request>,
        res: Arc<Response>,
        next: Arc<NextMiddleware>,
    ) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>> {
        let path = &req.path();
        let method = &req.method();

        for route in &self.routes {
            if &route.method == method && &route.path == path {
                let handler = Arc::clone(&route.handler);
                let res_clone = Arc::clone(&res);
                return Box::pin(async move {
                    if let Err(e) = handler(req, res_clone, next).await {
                        res.set_status(500);
                        res.body(format!("Internal Server Error: {}", e));
                    }
                    Ok(())
                });
            }
        }

        for (nested_path, nested_router) in &self.nested_routers {
            if path.starts_with(nested_path) {
                return nested_router.route(req, res, next);
            }
        }

        Box::pin(async move {
            res.set_status(404);
            res.body("Not Found".to_string());
            Ok(())
        })
    }

    pub fn handle(
        &self,
        req: Arc<Request>,
        res: Arc<Response>,
        next: Arc<NextMiddleware>,
    ) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>> {
        self.route(req, res, next)
    }

    pub fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::NextMiddleware;
    use suika_utils::noop_waker;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use std::task::Context;

    fn handler(
        req: Arc<Request>,
        res: Arc<Response>,
        next: Arc<NextMiddleware>,
    ) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>> {
        Box::pin(async move {
            res.body("Hello, world!".to_string());
            next.proceed(req, res).await?;
            Ok(())
        })
    }

    #[test]
    fn test_router_get() {
        let mut router = Router::new();
        router.get("/hello", handler);
        assert_eq!(router.routes.len(), 1);
        assert_eq!(router.routes[0].method, "GET");
        assert_eq!(router.routes[0].path, "/hello");
    }

    #[test]
    fn test_router_post() {
        let mut router = Router::new();
        router.post("/hello", handler);
        assert_eq!(router.routes.len(), 1);
        assert_eq!(router.routes[0].method, "POST");
        assert_eq!(router.routes[0].path, "/hello");
    }

    #[test]
    fn test_router_put() {
        let mut router = Router::new();
        router.put("/hello", handler);
        assert_eq!(router.routes.len(), 1);
        assert_eq!(router.routes[0].method, "PUT");
        assert_eq!(router.routes[0].path, "/hello");
    }

    #[test]
    fn test_router_delete() {
        let mut router = Router::new();
        router.delete("/hello", handler);
        assert_eq!(router.routes.len(), 1);
        assert_eq!(router.routes[0].method, "DELETE");
        assert_eq!(router.routes[0].path, "/hello");
    }

    #[test]
    fn test_router_patch() {
        let mut router = Router::new();
        router.patch("/hello", handler);
        assert_eq!(router.routes.len(), 1);
        assert_eq!(router.routes[0].method, "PATCH");
        assert_eq!(router.routes[0].path, "/hello");
    }

    #[test]
    fn test_use_router() {
        let mut main_router = Router::new();
        let nested_router = Router::new();
        main_router.use_router("/api", nested_router);
        assert_eq!(main_router.nested_routers.len(), 1);
        assert_eq!(main_router.nested_routers[0].0, "/api");
    }

    #[test]
    fn test_handle_get() {
        let mut router = Router::new();
        router.get("/hello", handler);

        let req =
            Arc::new(Request::new("GET /hello HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap());
        let res = Arc::new(Response::new());
        let next = Arc::new(NextMiddleware::new(Arc::new(Mutex::new(vec![]))));

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        let mut future = Box::pin(router.handle(req.clone(), res.clone(), next.clone()));

        while future.as_mut().poll(&mut context).is_pending() {}

        let body = res.get_body().map(|b| String::from_utf8(b).unwrap());
        assert_eq!(body, Some("Hello, world!".to_string()));
    }

    #[test]
    fn test_handle_post() {
        let mut router = Router::new();
        router.post("/hello", handler);

        let req =
            Arc::new(Request::new("POST /hello HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap());
        let res = Arc::new(Response::new());
        let next = Arc::new(NextMiddleware::new(Arc::new(Mutex::new(vec![]))));

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        let mut future = Box::pin(router.handle(req.clone(), res.clone(), next.clone()));

        while future.as_mut().poll(&mut context).is_pending() {}

        let body = res.get_body().map(|b| String::from_utf8(b).unwrap());
        assert_eq!(body, Some("Hello, world!".to_string()));
    }

    #[test]
    fn test_handle_not_found() {
        let mut router = Router::new();
        router.get("/hello", handler);

        let req = Arc::new(
            Request::new("GET /nonexistent HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap(),
        );

        let res = Arc::new(Response::new());
        let next = Arc::new(NextMiddleware::new(Arc::new(Mutex::new(vec![]))));

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        let mut future = Box::pin(router.handle(req.clone(), res.clone(), next.clone()));

        while future.as_mut().poll(&mut context).is_pending() {}

        let body = res.get_body().map(|b| String::from_utf8(b).unwrap());
        assert_eq!(body, Some("Not Found".to_string()));
        assert_eq!(res.get_status(), 404);
    }
}
