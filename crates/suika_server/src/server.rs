use crate::middleware::{Middleware, Next};
use crate::request::Request;
use crate::response::Response;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use suika_templates::TemplateEngine;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::runtime::{Builder, Handle};
use tokio::sync::oneshot;

/// Represents an HTTP server with middleware support.
pub struct Server {
    address: String,
    middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>>,
    template_engine: Option<TemplateEngine>,
    modules: Arc<Mutex<HashMap<String, Arc<dyn std::any::Any + Send + Sync>>>>,
    shutdown_signal: Option<Arc<Mutex<Option<oneshot::Sender<()>>>>>,
}

impl Server {
    /// Creates a new `Server` with the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to bind the server to.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::server::Server;
    ///
    /// let server = Server::new("127.0.0.1:8080");
    /// ```
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            middleware_stack: Vec::new(),
            template_engine: None,
            modules: Arc::new(Mutex::new(HashMap::new())),
            shutdown_signal: None,
        }
    }

    /// Adds middleware to the server.
    ///
    /// # Arguments
    ///
    /// * `mw` - An Arc containing the middleware to add.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::server::Server;
    /// use suika_server::middleware::{Middleware, Next};
    /// use suika_server::request::Request;
    /// use suika_server::response::Response;
    /// use std::sync::Arc;
    /// use std::pin::Pin;
    /// use std::future::Future;
    /// use suika_server::error::HttpError;
    ///
    /// struct ExampleMiddleware;
    ///
    /// impl Middleware for ExampleMiddleware {
    ///     fn handle<'a>(&'a self, req: &'a mut Request, res: &'a mut Response, mut next: Next<'a>) -> Pin<Box<dyn Future<Output = Result<(), HttpError>> + Send + 'a>> {
    ///         Box::pin(async move {
    ///             // Middleware logic here
    ///             next.run(req, res).await
    ///         })
    ///     }
    /// }
    ///
    /// let mut server = Server::new("127.0.0.1:8080");
    /// server.use_middleware(Arc::new(ExampleMiddleware));
    /// ```
    pub fn use_middleware(&mut self, mw: Arc<dyn Middleware + Send + Sync>) {
        self.middleware_stack.push(mw);
    }

    /// Sets the template engine to be used by the server.
    ///
    /// # Arguments
    ///
    /// * `engine` - The template engine to use.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::server::Server;
    /// use suika_templates::template_engine::TemplateEngine;
    ///
    /// let mut server = Server::new("127.0.0.1:8080");
    /// let template_engine = TemplateEngine::new();
    /// server.use_templates(template_engine);
    /// ```
    pub fn use_templates(&mut self, engine: TemplateEngine) {
        self.template_engine = Some(engine);
    }

    /// Adds a module to the server.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the module.
    /// * `module` - The module to add.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use suika_server::server::Server;
    /// use std::sync::Arc;
    ///
    /// let mut server = Server::new("127.0.0.1:8080");
    /// let my_database_store = Arc::new(MyDatabaseStore::new());
    /// server.use_module("store", my_database_store);
    /// ```
    pub fn use_module<T: 'static + Send + Sync>(&mut self, name: &str, module: T) {
        let mut modules = self.modules.lock().unwrap();
        modules.insert(name.to_string(), Arc::new(module));
    }

    /// Runs the server. If an existing runtime handle is provided, it is used to run the server.
    ///
    /// # Arguments
    ///
    /// * `existing_runtime` - An optional reference to an existing tokio runtime handle.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use suika_server::server::Server;
    ///
    /// fn main() {
    ///     let mut server = Server::new("127.0.0.1:8080");
    ///     server.run(None);
    ///     server.stop();
    /// }
    /// ```
    pub fn run(&mut self, existing_runtime: Option<&Handle>) {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        self.shutdown_signal = Some(Arc::new(Mutex::new(Some(shutdown_tx))));

        let address = self.address.clone();
        let middleware_stack = self.middleware_stack.clone();
        let template_engine = self.template_engine.clone();
        let modules = Arc::clone(&self.modules);

        if let Some(handle) = existing_runtime {
            handle.spawn(async move {
                Server::run_server(
                    address,
                    middleware_stack,
                    template_engine,
                    modules,
                    shutdown_rx,
                )
                .await;
            });
        } else {
            let num_cores = thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1);

            let runtime = Builder::new_multi_thread()
                .worker_threads(num_cores)
                .enable_all()
                .build()
                .unwrap();

            runtime.block_on(async move {
                Server::run_server(
                    address,
                    middleware_stack,
                    template_engine,
                    modules,
                    shutdown_rx,
                )
                .await;
            });
        }
    }

    /// Runs the server, listening for incoming TCP connections.
    ///
    /// # Arguments
    ///
    /// * `address` - The address on which the server will listen for incoming connections.
    /// * `middleware_stack` - A vector of middleware to be applied to each request.
    /// * `template_engine` - An optional template engine for rendering responses.
    /// * `modules` - A collection of modules that can be used by the server.
    /// * `shutdown_rx` - A receiver for the shutdown signal.
    async fn run_server(
        address: String,
        middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>>,
        template_engine: Option<TemplateEngine>,
        modules: Arc<Mutex<HashMap<String, Arc<dyn std::any::Any + Send + Sync>>>>,
        mut shutdown_rx: oneshot::Receiver<()>,
    ) {
        let listener = TcpListener::bind(&address)
            .await
            .expect("Failed to bind address");

        println!("Server running on {}", address);

        loop {
            tokio::select! {
                _ = &mut shutdown_rx => {
                    println!("Server shutting down");
                    break;
                }
                Ok((mut stream, _)) = listener.accept() => {
                    let mw_stack = middleware_stack.clone();
                    let tmpl_engine = template_engine.clone().map(Arc::new);
                    let modules = Arc::clone(&modules);

                    tokio::spawn(async move {
                        let mut buffer = [0; 1024];
                        if let Ok(size) = stream.read(&mut buffer).await {
                            if size > 0 {
                                let request_str = String::from_utf8_lossy(&buffer[..size]);
                                let mut req = Request::new(&request_str, Arc::clone(&modules)).unwrap();
                                let mut res = Response::new(tmpl_engine.clone());

                                let mut next = Next::new(&mw_stack);
                                if let Err(e) = next.run(&mut req, &mut res).await {
                                    res.error(e).await;
                                }

                                let status = res.status().await;
                                if status == None {
                                    res.set_status(404).await;
                                    res.body("404 Not Found".to_string()).await;
                                }

                                res.send(&mut stream).await.unwrap();
                            }
                        }
                    });
                }
                else => eprintln!("Failed to accept connection"),
            }
        }
    }

    /// Stops the running server by sending a shutdown signal.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use suika_server::server::Server;
    ///
    /// fn main() {
    ///     // Create a new server instance
    ///     let mut server = Server::new("127.0.0.1:8080");
    ///
    ///     // Run the server with a new Tokio runtime
    ///     server.run(None);
    ///
    ///     // Simulate some work being done
    ///     std::thread::sleep(std::time::Duration::from_secs(5));
    ///
    ///     // Stop the server
    ///     server.stop();
    /// }
    /// ```
    pub fn stop(&self) {
        if let Some(shutdown_signal) = &self.shutdown_signal {
            if let Ok(mut signal) = shutdown_signal.lock() {
                if let Some(tx) = signal.take() {
                    let _ = tx.send(());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::{Middleware, MiddlewareFuture, Next};
    use crate::request::Request;
    use crate::response::Response;
    use std::sync::Arc;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;
    use tokio::sync::Mutex;

    // Mock Middleware
    #[derive(Clone)]
    struct MockMiddleware {
        called: Arc<Mutex<bool>>,
    }

    impl MockMiddleware {
        fn new() -> Self {
            Self {
                called: Arc::new(Mutex::new(false)),
            }
        }
    }

    impl Middleware for MockMiddleware {
        fn handle<'a>(
            &'a self,
            req: &'a mut Request,
            res: &'a mut Response,
            mut next: Next<'a>,
        ) -> MiddlewareFuture<'a> {
            let called = Arc::clone(&self.called);
            Box::pin(async move {
                println!("MockMiddleware called");
                {
                    let mut called_lock = called.lock().await;
                    *called_lock = true;
                }
                res.set_status(200).await;
                res.body("Mock response".to_string()).await;
                next.run(req, res).await
            })
        }
    }

    #[tokio::test]
    async fn test_server_with_middleware() {
        let address = "127.0.0.1:8081";
        let mut server = Server::new(address);
        let mock_middleware = MockMiddleware::new();
        server.use_middleware(Arc::new(mock_middleware.clone()));

        let runtime_handle = tokio::runtime::Handle::current();
        server.run(Some(&runtime_handle));

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let mut stream = TcpStream::connect(address).await.unwrap();
        stream.write_all(b"GET / HTTP/1.1\r\n\r\n").await.unwrap();

        let mut buffer = [0; 1024];
        let size = stream.read(&mut buffer).await.unwrap();
        let response_str = String::from_utf8_lossy(&buffer[..size]);

        assert!(
            response_str.contains("Mock response"),
            "Response: {}",
            response_str
        );

        let called = *mock_middleware.called.lock().await;
        assert!(called);

        server.stop();
    }

    #[tokio::test]
    async fn test_server_without_middleware() {
        let address = "127.0.0.1:8082";
        let mut server = Server::new(address);
        let runtime_handle = tokio::runtime::Handle::current();

        server.run(Some(&runtime_handle));

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let mut stream = TcpStream::connect(address).await.unwrap();
        stream.write_all(b"GET / HTTP/1.1\r\n\r\n").await.unwrap();

        let mut buffer = [0; 1024];
        let size = stream.read(&mut buffer).await.unwrap();
        let response_str = String::from_utf8_lossy(&buffer[..size]);

        assert!(response_str.contains("404 Not Found"));

        server.stop();
    }
}
