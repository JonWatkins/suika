use crate::error::HttpError;
use crate::request::Request;
use crate::response::Response;
use futures::future::BoxFuture;
use std::sync::Arc;

pub type MiddlewareFuture<'a> = BoxFuture<'a, Result<(), HttpError>>;

/// A trait representing an HTTP middleware component.
///
/// Middleware components can inspect and modify requests and responses,
/// and pass control to the next middleware in the stack.
pub trait Middleware: Send + Sync {
    /// Handles an incoming HTTP request.
    ///
    /// # Arguments
    ///
    /// * `req` - A mutable reference to the incoming request.
    /// * `res` - A mutable reference to the response to be sent.
    /// * `next` - The next middleware in the stack.
    ///
    /// # Returns
    ///
    /// A future that resolves to a `Result<(), HttpError>`.
    fn handle<'a>(
        &'a self,
        req: &'a mut Request,
        res: &'a mut Response,
        next: Next<'a>,
    ) -> MiddlewareFuture<'a>;
}

/// Represents the next middleware in the stack.
#[derive(Clone)]
pub struct Next<'a> {
    index: usize,
    middleware_stack: &'a [Arc<dyn Middleware + Send + Sync>],
}

impl<'a> Next<'a> {
    /// Creates a new `Next` instance.
    ///
    /// # Arguments
    ///
    /// * `stack` - A slice of middleware components.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::middleware::{Next, LoggerMiddleware, Middleware};
    /// use std::sync::Arc;
    ///
    /// let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> = vec![Arc::new(LoggerMiddleware)];
    /// let next = Next::new(middleware_stack.as_slice());
    /// ```
    pub fn new(stack: &'a [Arc<dyn Middleware + Send + Sync>]) -> Self {
        Self {
            index: 0,
            middleware_stack: stack,
        }
    }

    /// Runs the next middleware in the stack.
    ///
    /// # Arguments
    ///
    /// * `req` - A mutable reference to the incoming request.
    /// * `res` - A mutable reference to the response to be sent.
    ///
    /// # Returns
    ///
    /// A future that resolves to a `Result<(), HttpError>`.
    pub fn run<'b>(
        &'b mut self,
        req: &'b mut Request,
        res: &'b mut Response,
    ) -> MiddlewareFuture<'b> {
        if self.index < self.middleware_stack.len() {
            let mw = &self.middleware_stack[self.index];
            self.index += 1;
            mw.handle(req, res, self.clone())
        } else {
            Box::pin(async { Ok(()) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::Mutex;

    // Mock Middleware
    struct MockMiddleware {
        counter: Arc<Mutex<i32>>,
    }

    impl MockMiddleware {
        fn new(counter: Arc<Mutex<i32>>) -> Self {
            Self { counter }
        }
    }

    impl Middleware for MockMiddleware {
        fn handle<'a>(
            &'a self,
            _req: &'a mut Request,
            _res: &'a mut Response,
            mut next: Next<'a>,
        ) -> MiddlewareFuture<'a> {
            let counter = Arc::clone(&self.counter);
            Box::pin(async move {
                let mut count = counter.lock().await;
                *count += 1;
                next.run(_req, _res).await
            })
        }
    }

    // Test Next::new
    #[test]
    fn test_next_new() {
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> =
            vec![Arc::new(MockMiddleware::new(Arc::new(Mutex::new(0))))];
        let next = Next::new(middleware_stack.as_slice());
        assert_eq!(next.index, 0);
        assert_eq!(next.middleware_stack.len(), 1);
    }

    // Test Next::run
    #[tokio::test]
    async fn test_next_run() {
        let counter = Arc::new(Mutex::new(0));
        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> =
            vec![Arc::new(MockMiddleware::new(Arc::clone(&counter)))];
        let mut next = Next::new(middleware_stack.as_slice());

        let mut req = Request::new("GET / HTTP/1.1\r\n\r\n").unwrap(); // Provide a valid HTTP request string
        let mut res = Response::new(); // Assuming Response::new() is a valid constructor

        next.run(&mut req, &mut res).await.unwrap();
        assert_eq!(*counter.lock().await, 1);
    }

    // Test Next::run with multiple middleware
    #[tokio::test]
    async fn test_next_run_multiple_middleware() {
        let counter1 = Arc::new(Mutex::new(0));
        let counter2 = Arc::new(Mutex::new(0));

        let middleware_stack: Vec<Arc<dyn Middleware + Send + Sync>> = vec![
            Arc::new(MockMiddleware::new(Arc::clone(&counter1))),
            Arc::new(MockMiddleware::new(Arc::clone(&counter2))),
        ];
        let mut next = Next::new(middleware_stack.as_slice());

        let mut req = Request::new("GET / HTTP/1.1\r\n\r\n").unwrap(); // Provide a valid HTTP request string
        let mut res = Response::new(); // Assuming Response::new() is a valid constructor

        next.run(&mut req, &mut res).await.unwrap();
        assert_eq!(*counter1.lock().await, 1);
        assert_eq!(*counter2.lock().await, 1);
    }
}
