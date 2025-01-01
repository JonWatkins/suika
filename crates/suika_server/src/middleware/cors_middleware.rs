use crate::NextMiddleware;
use crate::http::request::Request;
use crate::http::response::Response;
use crate::HttpError;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub fn cors_middleware(
    req: Arc<Request>,
    res: Arc<Response>,
    next: Arc<NextMiddleware>,
) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>> {
    Box::pin(async move {
        res.header("Access-Control-Allow-Origin", "*");
        res.header(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        );
        res.header(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        );

        if req.method() == "OPTIONS" {
            res.set_status(204);
            return Ok(());
        }

        next.proceed(req, res).await
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::{MiddlewareFn, NextMiddleware};
    use suika_utils::noop_waker;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};

    #[test]
    fn test_cors_middleware() {
        let middlewares: Vec<Arc<MiddlewareFn>> = vec![Arc::new(cors_middleware)];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));

        let req = Arc::new(Request::new("GET / HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap());
        let res = Arc::new(Response::new());
        let next = Arc::new(next_middleware);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        let mut future = Box::pin(cors_middleware(req.clone(), res.clone(), next.clone()));

        while let Poll::Pending = future.as_mut().poll(&mut context) {}

        assert_eq!(
            res.get_header("Access-Control-Allow-Origin"),
            Some("*".to_string())
        );
        assert_eq!(
            res.get_header("Access-Control-Allow-Methods"),
            Some("GET, POST, PUT, DELETE, OPTIONS".to_string())
        );
        assert_eq!(
            res.get_header("Access-Control-Allow-Headers"),
            Some("Content-Type, Authorization".to_string())
        );
        assert_eq!(res.get_status(), 200);
    }

    #[test]
    fn test_cors_middleware_options_request() {
        let middlewares: Vec<Arc<MiddlewareFn>> = vec![Arc::new(cors_middleware)];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));

        let req =
            Arc::new(Request::new("OPTIONS / HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap());
        let res = Arc::new(Response::new());
        let next = Arc::new(next_middleware);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(cors_middleware(req.clone(), res.clone(), next.clone()));

        while let Poll::Pending = future.as_mut().poll(&mut context) {}

        assert_eq!(
            res.get_header("Access-Control-Allow-Origin"),
            Some("*".to_string())
        );
        assert_eq!(
            res.get_header("Access-Control-Allow-Methods"),
            Some("GET, POST, PUT, DELETE, OPTIONS".to_string())
        );
        assert_eq!(
            res.get_header("Access-Control-Allow-Headers"),
            Some("Content-Type, Authorization".to_string())
        );
        assert_eq!(res.get_status(), 204);
    }
}
