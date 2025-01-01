use crate::NextMiddleware;
use core_http::{Request, Response};
use core_http_errors::HttpError;
use core_mime_type::get_mime_type;
use std::fs::File;
use std::future::Future;
use std::io::Read;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

fn map_io_error(err: std::io::Error) -> HttpError {
    HttpError::InternalServerError(format!("IO error: {}", err))
}

pub fn favicon_middleware(
    favicon_path: &str,
) -> impl Fn(
    Arc<Request>,
    Arc<Response>,
    Arc<NextMiddleware>,
) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>>
       + Send
       + Sync
       + 'static {
    let path = favicon_path.to_string();
    move |req: Arc<Request>, res: Arc<Response>, next: Arc<NextMiddleware>| {
        let path = path.clone();
        Box::pin(async move {
            if req.path() == "/favicon.ico" {
                let absolute_path = Path::new(&path).canonicalize();
                match absolute_path {
                    Ok(abs_path) => {
                        if !abs_path.exists() {
                            println!("File does not exist: {:?}", abs_path);
                            res.set_status(404);
                            res.body("Favicon not found".to_string());
                            return Err(HttpError::NotFound("Favicon not found".to_string()));
                        }

                        let mut file = match File::open(&abs_path) {
                            Ok(file) => file,
                            Err(_e) => {
                                res.set_status(404);
                                res.body("Favicon not found".to_string());
                                return Err(HttpError::NotFound("Favicon not found".to_string()));
                            }
                        };

                        let mut buffer = Vec::new();
                        if let Err(e) = file.read_to_end(&mut buffer) {
                            res.set_status(500);
                            res.body("Internal server error".to_string());
                            return Err(map_io_error(e));
                        }

                        res.body_bytes(buffer);

                        let mime_type = Path::new(&path)
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .map(get_mime_type)
                            .unwrap_or("application/octet-stream");
                        res.header("Content-Type", mime_type);

                        return Ok(());
                    }
                    Err(e) => {
                        println!("Failed to get absolute path: {:?}", e);
                        res.set_status(404);
                        res.body("Favicon not found".to_string());
                        return Err(HttpError::NotFound("Favicon not found".to_string()));
                    }
                }
            }

            next.proceed(req, res).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NextMiddleware;
    use core_http::{Request, Response};
    use std::fs::File;
    use std::io::Write;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, RawWaker, RawWakerVTable, Waker};

    fn noop_waker() -> Waker {
        fn noop(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(
                std::ptr::null(),
                &RawWakerVTable::new(clone, noop, noop, noop),
            )
        }
        unsafe { Waker::from_raw(clone(std::ptr::null())) }
    }

    #[test]
    fn test_favicon_found() {
        let favicon_path = "test_favicon.ico";
        let mut file = File::create(favicon_path).expect("Failed to create test favicon file");
        file.write_all(&[0; 256])
            .expect("Failed to write to test favicon file");

        let req = Arc::new(
            Request::new("GET /favicon.ico HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap(),
        );
        let res = Arc::new(Response::new());
        let next = Arc::new(NextMiddleware::new(Arc::new(Mutex::new(vec![]))));

        let middleware = favicon_middleware(favicon_path);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        let mut future = middleware(req.clone(), res.clone(), next.clone());

        while future.as_mut().poll(&mut context).is_pending() {}

        assert_eq!(res.get_status(), 200);
        assert_eq!(
            res.get_header("Content-Type"),
            Some("image/x-icon".to_string())
        );

        assert!(res.get_body().is_some());

        std::fs::remove_file(favicon_path).expect("Failed to remove test favicon file");
    }

    #[test]
    fn test_favicon_not_found() {
        let req = Arc::new(
            Request::new("GET /favicon.ico HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap(),
        );
        let res = Arc::new(Response::new());
        let next = Arc::new(NextMiddleware::new(Arc::new(Mutex::new(vec![]))));

        let favicon_path = "invalid/path/to/favicon.ico";

        let middleware = favicon_middleware(favicon_path);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        let mut future = middleware(req.clone(), res.clone(), next.clone());

        while future.as_mut().poll(&mut context).is_pending() {}

        assert_eq!(res.get_status(), 404);
        let body = res.get_body().map(|b| String::from_utf8(b).unwrap());
        assert_eq!(body, Some("Favicon not found".to_string()));
    }

    #[test]
    fn test_non_favicon_request() {
        let req = Arc::new(
            Request::new("GET /some_other_path HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap(),
        );
        let res = Arc::new(Response::new());
        let next = Arc::new(NextMiddleware::new(Arc::new(Mutex::new(vec![]))));

        let favicon_path = "public/favicon.ico";

        let middleware = favicon_middleware(favicon_path);

        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        let mut future = middleware(req.clone(), res.clone(), next.clone());

        while future.as_mut().poll(&mut context).is_pending() {}

        assert_eq!(res.get_status(), 200);
    }
}
