use crate::middleware::{MiddlewareFn, NextMiddleware};
use std::sync::{Arc, Mutex};

pub fn combine_middlewares(middlewares: Vec<Arc<MiddlewareFn>>) -> Arc<MiddlewareFn> {
    Arc::new(move |req, res, _next| {
        let middlewares = middlewares.clone();
        let next = Arc::new(NextMiddleware::new(Arc::new(Mutex::new(middlewares))));
        Box::pin(async move { next.proceed(req, res).await })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::{cors_middleware, logger_middleware};
    use crate::http::request::Request;
    use crate::http::response::Response;
    use suika_utils::noop_waker;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};

    #[test]
    fn test_combine_middlewares() {
        let middlewares: Vec<Arc<MiddlewareFn>> =
            vec![Arc::new(logger_middleware), Arc::new(cors_middleware)];

        let combined_middleware = combine_middlewares(middlewares);

        let req = Arc::new(Request::new("GET / HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap());
        let res = Arc::new(Response::new());
        let next = Arc::new(NextMiddleware::new(Arc::new(Mutex::new(vec![]))));

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut future = combined_middleware(req.clone(), res.clone(), next.clone());

        while let Poll::Pending = Future::poll(Pin::as_mut(&mut future), &mut context) {}

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
    }
}
