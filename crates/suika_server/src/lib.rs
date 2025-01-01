use suika_async::SimpleAsync;
use suika_http::{Request, Response};
use suika_errors::HttpError;
use suika_middleware::{Middleware, NextMiddleware};
use std::future::Future;
use std::io::Read;
use std::net::TcpListener;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    middlewares: Arc<Mutex<Vec<Middleware>>>,
    handler: Option<
        Arc<
            dyn Fn(
                    Arc<Request>,
                    Arc<Response>,
                ) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>>
                + Send
                + Sync,
        >,
    >,
}

impl Server {
    pub fn new() -> Self {
        Self {
            middlewares: Arc::new(Mutex::new(vec![])),
            handler: None,
        }
    }

    pub fn use_middleware<F, Fut>(&self, middleware: F)
    where
        F: Fn(Arc<Request>, Arc<Response>, Arc<NextMiddleware>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), HttpError>> + Send + 'static,
    {
        self.middlewares
            .lock()
            .unwrap()
            .push(Arc::new(move |req, res, next| {
                Box::pin(middleware(req, res, next))
            }));
    }

    pub fn handle<F, Fut>(&mut self, handler: F)
    where
        F: Fn(Arc<Request>, Arc<Response>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), HttpError>> + Send + 'static,
    {
        self.handler = Some(Arc::new(move |req, res| Box::pin(handler(req, res))));
    }

    pub fn listen(&self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        println!("Server running on {}", addr);

        let simple_async = Arc::new(SimpleAsync::new());
        let simple_async_clone = Arc::clone(&simple_async);

        thread::spawn(move || {
            simple_async_clone.run();
        });

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_connection(stream, Arc::clone(&simple_async)),
                Err(e) => eprintln!("Failed to establish a connection: {}", e),
            }
        }
    }

    fn handle_connection(&self, mut stream: std::net::TcpStream, simple_async: Arc<SimpleAsync>) {
        let middlewares = Arc::clone(&self.middlewares);
        let handler = self.handler.clone();

        simple_async.spawn(async move {
            if let Err(e) = Self::process_stream(&mut stream, middlewares, handler).await {
                eprintln!("Error processing stream: {}", e);
            }
        });
    }

    async fn process_stream(
        stream: &mut std::net::TcpStream,
        middlewares: Arc<Mutex<Vec<Middleware>>>,
        handler: Option<
            Arc<
                dyn Fn(
                        Arc<Request>,
                        Arc<Response>,
                    )
                        -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>>
                    + Send
                    + Sync,
            >,
        >,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        let request_string = String::from_utf8_lossy(&buffer[..]).to_string();
        let req = Arc::new(Request::new(&request_string)?);
        let res = Arc::new(Response::new());
        let next = Arc::new(NextMiddleware::new(middlewares));

        Self::process_request(req.clone(), res.clone(), next, handler).await?;

        res.send(stream)?;
        Ok(())
    }

    async fn process_request(
        req: Arc<Request>,
        res: Arc<Response>,
        next: Arc<NextMiddleware>,
        handler: Option<
            Arc<
                dyn Fn(
                        Arc<Request>,
                        Arc<Response>,
                    )
                        -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>>
                    + Send
                    + Sync,
            >,
        >,
    ) -> Result<(), Box<dyn std::error::Error>> {
        next.proceed(req.clone(), res.clone()).await?;
        if let Some(handler) = handler {
            handler(req, res.clone()).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use suika_http::{Request, Response};
    use suika_errors::HttpError;
    use suika_middleware::NextMiddleware;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;

    fn example_middleware(
        req: Arc<Request>,
        res: Arc<Response>,
        next: Arc<NextMiddleware>,
    ) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>> {
        Box::pin(async move {
            res.header("X-Middleware", "Processed");
            next.proceed(req, res).await
        })
    }

    fn example_handler(
        _req: Arc<Request>,
        res: Arc<Response>,
    ) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send>> {
        Box::pin(async move {
            res.body("Hello, world!".to_string());
            Ok(())
        })
    }

    #[test]
    fn test_use_middleware() {
        let server = Server::new();
        server.use_middleware(example_middleware);

        assert_eq!(server.middlewares.lock().unwrap().len(), 1);
    }

    #[test]
    fn test_handle() {
        let mut server = Server::new();
        server.handle(example_handler);

        assert!(server.handler.is_some());
    }
}
