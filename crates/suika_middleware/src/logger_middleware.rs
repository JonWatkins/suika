use crate::NextMiddleware;
use suika_http::{Request, Response};
use suika_errors::HttpError;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub fn logger_middleware(
    req: Arc<Request>,
    res: Arc<Response>,
    next: Arc<NextMiddleware>,
) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>> {
    Box::pin(async move {
        println!("Request: {} {}", req.method(), req.path());
        let result = next.proceed(req.clone(), res.clone()).await;
        println!("Response: {}", res.get_status());
        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MiddlewareFn, NextMiddleware};
    use suika_http::{Request, Response};
    use std::future::Future;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};
    use std::task::{RawWaker, RawWakerVTable, Waker};

    fn noop_waker() -> Waker {
        fn noop(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
        unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
    }

    #[test]
    fn test_logger_middleware() {
        let middlewares: Vec<Arc<MiddlewareFn>> = vec![Arc::new(logger_middleware)];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));

        let req = Arc::new(Request::new("GET / HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap());
        let res = Arc::new(Response::new());
        let next = Arc::new(next_middleware);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(logger_middleware(req.clone(), res.clone(), next.clone()));

        while let Poll::Pending = future.as_mut().poll(&mut context) {}

        assert_eq!(res.get_status(), 200);
    }

    #[test]
    fn test_logger_middleware_with_next() {
        let middlewares: Vec<Arc<MiddlewareFn>> = vec![
            Arc::new(logger_middleware),
            Arc::new(|req, res, next| {
                Box::pin(async move {
                    res.body("Hello, world!".to_string());
                    next.proceed(req, res).await
                })
            }),
        ];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));

        let req = Arc::new(Request::new("GET / HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap());
        let res = Arc::new(Response::new());
        let next = Arc::new(next_middleware);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(logger_middleware(req.clone(), res.clone(), next.clone()));

        while let Poll::Pending = future.as_mut().poll(&mut context) {}

        let body = res.get_body().map(|b| String::from_utf8(b).unwrap());
        assert_eq!(body, Some("Hello, world!".to_string()));
    }
}
