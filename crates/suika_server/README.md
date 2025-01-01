# Suika Server

Suika Server is a core component for handling HTTP requests and responses in the
Suika web stack.

**Note:** Suika is under active development and not intended for production use.
The API is subject to change and may lack comprehensive testing and
documentation.

## Library Features

- HTTP Handling
  - **Request**: Represents an HTTP request.
  - **Response**: Represents an HTTP response.
  - **HttpError**: Represents errors that can occur during HTTP handling.
- Middleware
  - **combine_middlewares**: Combines multiple middleware functions.
  - **cors_middleware**: Middleware for handling CORS (Cross-Origin Resource
    Sharing).
  - **favicon_middleware**: Middleware for serving a favicon.
  - **logger_middleware**: Middleware for logging HTTP requests and responses.
  - **static_file_middleware**: Middleware for serving static files.
  - **MiddlewareFn**: Type alias for a middleware function.
  - **NextMiddleware**: Represents the next middleware in the chain.
- Routing
  - **Router**: Represents the routing logic for handling different HTTP routes.
- Server
  - **Server**: Represents the HTTP server.

## Example usage

```rust
use suika::server::{router::Router, Server};
use std::sync::Arc;

pub fn main() {
    let server = Server::new();
    let mut router = Router::new();

    router.get("/", |_req, res, _next| async move {
        res.set_status(200);
        res.body("Hello World".to_string());
        Ok(())
    });

    let router = Arc::new(router);

    server.use_middleware(move |req, res, next| {
        let router = Arc::clone(&router);
        Box::pin(async move { router.handle(req, res, next).await })
    });

    server.listen("127.0.0.1:7878");
}
```
