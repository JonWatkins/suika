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
  - **CorsMiddleware**: Middleware for handling CORS (Cross-Origin Resource
    Sharing).
  - **FaviconMiddleware**: Middleware for serving a favicon.
  - **LoggerMiddleware**: Middleware for logging HTTP requests and responses.
  - **StaticFileMiddleware**: Middleware for serving static files.
- Routing
  - **Router**: Represents the routing logic for handling different HTTP routes.
- Server
  - **Server**: Represents the HTTP server.

## Example usage

```rust
use suika::server::{Server, Router};
use std::sync::Arc;

pub fn main() {
    let mut server = Server::new("127.0.0.1:8080");
    let mut router = Router::new("/");

    router.add_route(Some("GET"), r"/?$", |_req, res| {
        Box::pin(async move {
            res.set_status(201).await;
            res.body("Hello World!".to_string()).await;
            Ok(())
        })
    });

    server.use_middleware(Arc::new(router));

    server.run();
}
```
