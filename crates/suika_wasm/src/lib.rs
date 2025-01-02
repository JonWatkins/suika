use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use suika_mime::get_mime_type;
use suika_server::{
    http::{error::HttpError, request::Request, response::Response},
    middleware::NextMiddleware,
};

/// Middleware to serve WebAssembly (Wasm) files from a specified directory.
///
/// # Arguments
///
/// * `url_prefix` - The URL prefix to match for serving Wasm files.
/// * `cache_duration` - The duration (in seconds) for which the Wasm files should be cached.
///
/// # Returns
///
/// A function that can be used as middleware to serve Wasm files.
///
/// # Examples
///
/// ```
/// use suika_server::{
///     http::{request::Request, response::Response, error::HttpError},
///     middleware::{NextMiddleware, MiddlewareFn},
/// };
/// use suika_wasm::wasm_file_middleware;
/// use suika_utils::noop_waker;
/// use std::sync::{Arc, Mutex};
/// use std::future::Future;
/// use std::pin::Pin;
/// use std::task::{Context, Poll};
///
/// let middleware = wasm_file_middleware("/wasm", 3600);
///
/// let req = Arc::new(Request::new("GET /wasm/suika_ui_bg.wasm HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap());
/// let res = Arc::new(Response::new());
/// let next = Arc::new(NextMiddleware::new(Arc::new(Mutex::new(vec![]))));
///
/// let waker = noop_waker();
/// let mut context = Context::from_waker(&waker);
///
/// let mut future = Box::pin(middleware(req, res, next));
///
/// while let Poll::Pending = future.as_mut().poll(&mut context) {}
/// ```
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
        let path = if let Some(stripped_path) = req.path().strip_prefix(url_prefix) {
            format!("crates/suika_wasm/wasm/{}", stripped_path)
        } else {
            return Box::pin(async move { next.proceed(req, res).await });
        };

        Box::pin(async move {
            if Path::new(&path).exists() {
                if let Err(e) = res.send_file(&path) {
                    res.set_status(500);
                    res.body(format!("Internal Server Error: {}", e));
                } else {
                    let mime_type = Path::new(&path)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(get_mime_type)
                        .unwrap_or_else(|| "application/wasm".to_string());
                    res.header("Content-Type", mime_type.as_str());

                    res.header(
                        "Cache-Control",
                        &format!("public, max-age={}", cache_duration),
                    );
                }
                Ok(())
            } else {
                println!("File not found: {}", path);
                next.proceed(req, res).await
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};
    use suika_server::middleware::MiddlewareFn;
    use suika_utils::noop_waker;

    #[test]
    fn test_wasm_file_middleware_file_exists() {
        let wasm_dir = "crates/suika_wasm/wasm";
        let wasm_file_path = format!("{}/suika_ui_bg.wasm", wasm_dir);

        fs::create_dir_all(wasm_dir).unwrap();
        let mut file = File::create(&wasm_file_path).unwrap();
        file.write_all(b"\0asm\x01\0\0\0").unwrap();

        let middlewares: Vec<Arc<MiddlewareFn>> =
            vec![Arc::new(wasm_file_middleware("/wasm", 3600))];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));

        let req = Arc::new(
            Request::new("GET /wasm/suika_ui_bg.wasm HTTP/1.1\r\nHost: example.com\r\n\r\n")
                .unwrap(),
        );
        let res = Arc::new(Response::new());
        let next = Arc::new(next_middleware);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(wasm_file_middleware("/wasm", 3600)(
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
        assert_eq!(res.get_body().is_some(), true);

        fs::remove_file(wasm_file_path).unwrap();
        fs::remove_dir_all(wasm_dir).unwrap();
    }

    #[test]
    fn test_wasm_file_middleware_file_not_exists() {
        let middlewares: Vec<Arc<MiddlewareFn>> =
            vec![Arc::new(wasm_file_middleware("/wasm", 3600))];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));

        let req = Arc::new(
            Request::new("GET /wasm/nonexistent.wasm HTTP/1.1\r\nHost: example.com\r\n\r\n")
                .unwrap(),
        );
        let res = Arc::new(Response::new());
        let next = Arc::new(next_middleware);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(wasm_file_middleware("/wasm", 3600)(
            req.clone(),
            res.clone(),
            next.clone(),
        ));

        while let Poll::Pending = future.as_mut().poll(&mut context) {}

        assert_eq!(res.get_header("Content-Type"), None);
        assert_eq!(res.get_header("Cache-Control"), None);
        assert_eq!(res.get_body().is_none(), true);
    }

    #[test]
    fn test_wasm_file_middleware_path_does_not_match() {
        let middlewares: Vec<Arc<MiddlewareFn>> =
            vec![Arc::new(wasm_file_middleware("/wasm", 3600))];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));

        let req = Arc::new(
            Request::new("GET /other/suika_ui_bg.wasm HTTP/1.1\r\nHost: example.com\r\n\r\n")
                .unwrap(),
        );
        let res = Arc::new(Response::new());
        let next = Arc::new(next_middleware);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(wasm_file_middleware("/wasm", 3600)(
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
