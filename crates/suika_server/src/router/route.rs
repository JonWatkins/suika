use regex::Regex;
use crate::http::request::Request;
use crate::http::response::Response;
use crate::HttpError;
use crate::middleware::NextMiddleware;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Clone)]
pub struct Route {
    pub path: String,
    pub method: String,
    pub regex: Regex,
    pub handler: Arc<
        dyn Fn(
                Arc<Request>,
                Arc<Response>,
                Arc<NextMiddleware>,
            ) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>>
            + Send
            + Sync,
    >,
}

impl Route {
    pub fn new<F, Fut>(method: &str, path: &str, handler: F) -> Self
    where
        F: Fn(Arc<Request>, Arc<Response>, Arc<NextMiddleware>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), HttpError>> + Send + 'static,
    {
        let regex = Regex::new(&format!("^{}$", path)).unwrap();
        Route {
            path: path.to_string(),
            method: method.to_string(),
            regex,
            handler: Arc::new(move |req, res, next| Box::pin(handler(req, res, next))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::NextMiddleware;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;

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
    fn test_create_route() {
        let route = Route::new("GET", "/hello", handler);
        assert_eq!(route.method, "GET");
        assert_eq!(route.path, "/hello");
    }
}
