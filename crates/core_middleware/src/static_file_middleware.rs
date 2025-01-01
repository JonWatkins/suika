use crate::NextMiddleware;
use core_http::{Request, Response};
use core_http_errors::HttpError;
use core_mime_type::get_mime_type;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

pub fn static_file_middleware(
    url_prefix: &'static str,
    directory: &'static str,
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
            format!("{}/{}", directory, stripped_path)
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
                        .unwrap_or("application/octet-stream");
                    res.header("Content-Type", mime_type);

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
    use crate::{MiddlewareFn, NextMiddleware};
    use core_http::{Request, Response};
    use std::fs::File;
    use std::future::Future;
    use std::io::Write;
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
    fn test_static_file_middleware_file_exists() {
        let static_dir = "public";
        let static_file_path = format!("{}/index.html", static_dir);
        std::fs::create_dir_all(static_dir).unwrap();
        let mut file = File::create(&static_file_path).unwrap();
        writeln!(file, "<html><body>Hello, world!</body></html>").unwrap();

        let middlewares: Vec<Arc<MiddlewareFn>> = vec![Arc::new(static_file_middleware(
            "/public/", static_dir, 3600,
        ))];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));

        let req = Arc::new(
            Request::new("GET /public/index.html HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap(),
        );
        let res = Arc::new(Response::new());
        let next = Arc::new(next_middleware);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(static_file_middleware("/public/", static_dir, 3600)(
            req.clone(),
            res.clone(),
            next.clone(),
        ));

        while let Poll::Pending = future.as_mut().poll(&mut context) {}

        assert_eq!(
            res.get_header("Content-Type"),
            Some("text/html".to_string())
        );
        assert_eq!(
            res.get_header("Cache-Control"),
            Some("public, max-age=3600".to_string())
        );
        assert_eq!(res.get_body().is_some(), true);

        std::fs::remove_file(static_file_path).unwrap();
        std::fs::remove_dir_all(static_dir).unwrap();
    }

    #[test]
    fn test_static_file_middleware_file_not_exists() {
        let middlewares: Vec<Arc<MiddlewareFn>> =
            vec![Arc::new(static_file_middleware("/public/", "public", 3600))];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));

        let req = Arc::new(
            Request::new("GET /public/nonexistent.html HTTP/1.1\r\nHost: example.com\r\n\r\n")
                .unwrap(),
        );
        let res = Arc::new(Response::new());
        let next = Arc::new(next_middleware);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(static_file_middleware("/public/", "public", 3600)(
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
    fn test_static_file_middleware_path_does_not_match() {
        let middlewares: Vec<Arc<MiddlewareFn>> =
            vec![Arc::new(static_file_middleware("/public/", "public", 3600))];
        let next_middleware = NextMiddleware::new(Arc::new(Mutex::new(middlewares)));

        let req = Arc::new(
            Request::new("GET /other/index.html HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap(),
        );
        let res = Arc::new(Response::new());
        let next = Arc::new(next_middleware);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(static_file_middleware("/public/", "public", 3600)(
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
