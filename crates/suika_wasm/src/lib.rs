use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use suika_server::{
    http::{error::HttpError, request::Request, response::Response},
    middleware::NextMiddleware,
};

const WASM_BINARY: &[u8] = include_bytes!("../wasm/suika_ui_bg.wasm");
const JS_FILE: &str = include_str!("../wasm/suika_ui.js");

/// Middleware to serve embedded WebAssembly (Wasm) and JavaScript files.
///
/// # Arguments
///
/// * `url_prefix` - The URL prefix to match for serving the files.
/// * `cache_duration` - The duration (in seconds) for which the files should be cached.
///
/// # Returns
///
/// A function that can be used as middleware to serve the files.
pub fn wasm_file_middleware(
    url_prefix: &'static str,
    cache_duration: u64,
) -> impl Fn(
    Arc<Request>,
    Arc<Response>,
    Arc<NextMiddleware>,
) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>>
       + Send
       + Sync
       + 'static {
    move |req: Arc<Request>, res: Arc<Response>, next: Arc<NextMiddleware>| {
        let path = req.path();

        if path == format!("{}/suika_ui_bg.wasm", url_prefix) {
            Box::pin(async move {
                res.header("Content-Type", "application/wasm");
                res.header(
                    "Cache-Control",
                    &format!("public, max-age={}", cache_duration),
                );
                res.body_bytes(WASM_BINARY.to_vec());
                Ok(())
            })
        } else if path == format!("{}/suika_ui.js", url_prefix) {
            Box::pin(async move {
                res.header("Content-Type", "application/javascript");
                res.header(
                    "Cache-Control",
                    &format!("public, max-age={}", cache_duration),
                );
                res.body(JS_FILE.to_string());
                Ok(())
            })
        } else {
            Box::pin(async move { next.proceed(req, res).await })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};
    use suika_server::middleware::MiddlewareFn;
    use suika_utils::noop_waker;

    #[test]
    fn test_wasm_and_js_file_middleware_path_matches_wasm() {
        let middlewares: Vec<Arc<MiddlewareFn>> =
            vec![Arc::new(wasm_file_middleware("/files", 3600))];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));

        let req = Arc::new(
            Request::new("GET /files/suika_ui_bg.wasm HTTP/1.1\r\nHost: example.com\r\n\r\n")
                .unwrap(),
        );
        let res = Arc::new(Response::new());
        let next = Arc::new(next_middleware);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(wasm_file_middleware("/files", 3600)(
            req.clone(),
            res.clone(),
            next.clone(),
        ));

        while let Poll::Pending = future.as_mut().poll(&mut context) {}

        assert_eq!(
            res.get_header("Content-Type"),
            Some("application/wasm".to_string())
        );
        assert_eq!(
            res.get_header("Cache-Control"),
            Some("public, max-age=3600".to_string())
        );
        assert!(res.get_body().is_some());
        assert_eq!(res.get_body().unwrap(), WASM_BINARY.to_vec());
    }

    #[test]
    fn test_wasm_and_js_file_middleware_path_matches_js() {
        let middlewares: Vec<Arc<MiddlewareFn>> =
            vec![Arc::new(wasm_file_middleware("/files", 3600))];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));

        let req = Arc::new(
            Request::new("GET /files/suika_ui.js HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap(),
        );
        let res = Arc::new(Response::new());
        let next = Arc::new(next_middleware);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(wasm_file_middleware("/files", 3600)(
            req.clone(),
            res.clone(),
            next.clone(),
        ));

        while let Poll::Pending = future.as_mut().poll(&mut context) {}

        assert_eq!(
            res.get_header("Content-Type"),
            Some("application/javascript".to_string())
        );
        assert_eq!(
            res.get_header("Cache-Control"),
            Some("public, max-age=3600".to_string())
        );
        assert!(res.get_body().is_some());
        assert_eq!(res.get_body().unwrap(), JS_FILE.as_bytes());
    }

    #[test]
    fn test_wasm_and_js_file_middleware_path_does_not_match() {
        let middlewares: Vec<Arc<MiddlewareFn>> =
            vec![Arc::new(wasm_file_middleware("/files", 3600))];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));

        let req = Arc::new(
            Request::new("GET /other/suika_ui_bg.wasm HTTP/1.1\r\nHost: example.com\r\n\r\n")
                .unwrap(),
        );
        let res = Arc::new(Response::new());
        let next = Arc::new(next_middleware);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(wasm_file_middleware("/files", 3600)(
            req.clone(),
            res.clone(),
            next.clone(),
        ));

        while let Poll::Pending = future.as_mut().poll(&mut context) {}

        assert_eq!(res.get_header("Content-Type"), None);
        assert_eq!(res.get_header("Cache-Control"), None);
        assert_eq!(res.get_body().is_none(), true);
    }
}
