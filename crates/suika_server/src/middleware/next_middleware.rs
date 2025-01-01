use crate::http::request::Request;
use crate::http::response::Response;
use crate::HttpError;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

pub type MiddlewareFn = dyn Fn(
        Arc<Request>,
        Arc<Response>,
        Arc<NextMiddleware>,
    ) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>>
    + Send
    + Sync;

pub struct NextMiddleware {
    middlewares: Arc<Mutex<Vec<Arc<MiddlewareFn>>>>,
    index: usize,
}

impl NextMiddleware {
    pub fn new(middlewares: Arc<Mutex<Vec<Arc<MiddlewareFn>>>>) -> Self {
        Self {
            middlewares,
            index: 0,
        }
    }

    pub async fn next(
        self: Arc<Self>,
        req: Arc<Request>,
        res: Arc<Response>,
    ) -> Result<(), HttpError> {
        let middleware = {
            let middlewares = self.middlewares.lock().unwrap();
            if self.index < middlewares.len() {
                Some(middlewares[self.index].clone())
            } else {
                None
            }
        };

        if let Some(middleware) = middleware {
            let next = Arc::new(Self {
                middlewares: Arc::clone(&self.middlewares),
                index: self.index + 1,
            });
            middleware(req, res, next).await
        } else {
            Ok(())
        }
    }

    pub async fn proceed(
        self: Arc<Self>,
        req: Arc<Request>,
        res: Arc<Response>,
    ) -> Result<(), HttpError> {
        self.next(req, res).await
    }
}

impl Clone for NextMiddleware {
    fn clone(&self) -> Self {
        NextMiddleware {
            middlewares: Arc::clone(&self.middlewares),
            index: self.index,
        }
    }
}

pub type Middleware = Arc<MiddlewareFn>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::logger_middleware;
    use suika_utils::noop_waker;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use std::task::Context;

    fn middleware_fn_1(
        req: Arc<Request>,
        res: Arc<Response>,
        next: Arc<NextMiddleware>,
    ) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>> {
        Box::pin(async move {
            res.header("X-Middleware-1", "Processed");
            next.proceed(req, res).await
        })
    }

    fn middleware_fn_2(
        req: Arc<Request>,
        res: Arc<Response>,
        next: Arc<NextMiddleware>,
    ) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>> {
        Box::pin(async move {
            res.header("X-Middleware-2", "Processed");
            next.proceed(req, res).await
        })
    }

    fn middleware_fn_error(
        _req: Arc<Request>,
        _res: Arc<Response>,
        _next: Arc<NextMiddleware>,
    ) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>> {
        Box::pin(async move {
            Err(HttpError::InternalServerError(
                "Middleware error".to_string(),
            ))
        })
    }

    #[test]
    fn test_next_middleware_chain() {
        let middlewares: Vec<Arc<MiddlewareFn>> = vec![
            Arc::new(logger_middleware),
            Arc::new(middleware_fn_1),
            Arc::new(middleware_fn_2),
        ];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));
        let req = Arc::new(Request::new("GET / HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap());
        let res = Arc::new(Response::new());

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        let mut future = Box::pin(Arc::new(next_middleware).proceed(req.clone(), res.clone()));

        while future.as_mut().poll(&mut context).is_pending() {}

        assert_eq!(
            res.get_header("X-Middleware-1"),
            Some("Processed".to_string())
        );
        assert_eq!(
            res.get_header("X-Middleware-2"),
            Some("Processed".to_string())
        );
    }

    #[test]
    fn test_middleware_error_handling() {
        let middlewares: Vec<Arc<MiddlewareFn>> = vec![
            Arc::new(logger_middleware),
            Arc::new(middleware_fn_1),
            Arc::new(middleware_fn_error),
            Arc::new(middleware_fn_2),
        ];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));
        let req = Arc::new(Request::new("GET / HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap());
        let res = Arc::new(Response::new());

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        let mut future = Box::pin(Arc::new(next_middleware).proceed(req.clone(), res.clone()));

        let result = loop {
            match future.as_mut().poll(&mut context) {
                std::task::Poll::Ready(res) => break res,
                std::task::Poll::Pending => (),
            }
        };

        assert!(result.is_err());
        if let Err(HttpError::InternalServerError(msg)) = result {
            assert_eq!(msg, "Middleware error");
        } else {
            panic!("Expected InternalServerError");
        }
    }
}
