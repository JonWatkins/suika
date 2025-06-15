# Suika Server: A Toy HTTP Server Component

**Important: This is a personal toy project, developed as an experiment and learning exercise. It is not intended for production use.**

**As a toy project, its future development is uncertain. It may or may not receive future updates, maintenance, or bug fixes. Please do not use it in production environments.**

Suika Server is a core component for handling HTTP requests and responses within the `Suika` web stack (also a toy project). It provides foundational elements for building a web server, primarily for understanding server-side concepts and for experimental purposes.

The API is subject to change. This project is not thoroughly tested or hardened for real-world applications, and documentation may be basic.

## Library Features

-   HTTP Handling
    -   **Request**: Represents an HTTP request.
    -   **Response**: Represents an HTTP response.
    -   **HttpError**: Represents errors that can occur during HTTP handling.
-   Middleware
    -   **CorsMiddleware**: Middleware for handling CORS (Cross-Origin Resource Sharing).
    -   **FaviconMiddleware**: Middleware for serving a favicon.
    -   **LoggerMiddleware**: Middleware for logging HTTP requests and responses.
    -   **StaticFileMiddleware**: Middleware for serving static files.
-   Routing
    -   **Router**: Represents the routing logic for handling different HTTP routes.
-   Server
    -   **Server**: Represents the HTTP server.

## Example usage

```rust
use suika::server::{Server, Router};
use std::sync::Arc;

pub fn main() {
    let mut server = Server::new("127.0.0.1:8080");
    let mut router = Router::new("/");

    router.get(r"/?$", |_req, res| {
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