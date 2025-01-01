# Suika

Suika is an evolving web stack currently under active development. At this
stage, it is not intended for production use. Significant work remains,
including API refinement, comprehensive documentation, and extensive testing.
The API is subject to change as this implementation is in its early stages and
requires substantial enhancements. For instance, the router currently performs
basic string matching, with plans to support regular expressions in future
updates.

Please refrain from using this version in production environments, as it has not
been thoroughly tested for bugs. The primary aim of this project is to serve as
a learning tool, and therefore, efforts have been made to minimize dependencies
on external crates.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
suika_templates = "0.1.1"
```

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
- MIME Type Handling
  - **get_mime_type**: Function to get the MIME type based on a file extension.
- Routing
  - **Router**: Represents the routing logic for handling different HTTP routes.
- Server
  - **Server**: Represents the HTTP server.
- Templates
  - **TemplateEngine**: Represents the template engine for rendering templates.
  - **TemplateParser**: Parses template strings.
  - **TemplateToken**: Represents a token in a template.
  - **TemplateValue**: Represents a value in a template.
- Utilities
  - **build_url**: Function to build a URL.
  - **parse_query_string**: Function to parse a query string.
  - **parse_url**: Function to parse a URL.

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

### Static file server

```rust
use suika::server::{middleware::static_file_middleware, Server};

pub fn main() {
    let server = Server::new();
    server.use_middleware(static_file_middleware("/", "public", 3200));
    server.listen("127.0.0.1:7878");
}
```

### Post Data

```rust
use suika::server::{router::Router, Server};
use std::sync::Arc;

pub fn main() {
    let server = Server::new();
    let mut router = Router::new();

    router.post("/json", |req, res, _next| async move {
        if let Some(json_body) = req.json_body() {
            let response_message = format!("Data received: {:?}\n", json_body);
            res.body(response_message);
        } else {
            res.set_status(400);
            res.body("Invalid JSON data received!\n".to_string());
        }
        Ok(())
    });

    router.post("/form", |req, res, _next| async move {
        if let Some(form_data) = req.form_data() {
            let response_message = format!("Form Data received: {:?}\n", form_data);
            res.body(response_message);
        } else {
            res.set_status(400);
            res.body("Invalid form data received!\n".to_string());
        }
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

### Sending files

```rust
use suika::server::{router::Router, Server};
use std::sync::Arc;

pub fn main() {
    let server = Server::new();
    let mut router = Router::new();

    router.get("/", |_req, res, _next| async move {
        if let Err(e) = res.send_file("index.html") {
            eprintln!("Error: {}", e);
        }
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

### Middleware

```rust
use suika::server::{
    middleware::{
        combine_middlewares, cors_middleware, favicon_middleware, logger_middleware,
        static_file_middleware,
    },
    router::Router,
    Server,
};

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

    let combined_middleware = combine_middlewares(vec![
        Arc::new(cors_middleware),
        Arc::new(favicon_middleware("public/favicon.ico")),
        Arc::new(static_file_middleware("/public", "public", 3600)),
        Arc::new(logger_middleware),
        Arc::new(move |req, res, next| {
            let router = Arc::clone(&router);
            Box::pin(async move { router.handle(req, res, next).await })
        }),
    ]);

    server.use_middleware(move |req, res, next| combined_middleware(req, res, next));
    server.listen("127.0.0.1:7878");
}
```

### Template Engine

```rust
use std::{collections::HashMap, sync::Arc};

use suika::{
    templates::{TemplateEngine, TemplateValue},
    server:: { Server, router::Router },
};

pub fn main() {
    let server = Server::new();
    let mut router = Router::new();

    let template_engine = Arc::new({
        let mut engine = TemplateEngine::new();

        engine
            .load_templates_from_directory("templates")
            .expect("Failed to load templates from directory");

        engine
    });

    router.get("/", move |_req, res, _next| {
        let template_engine = Arc::clone(&template_engine);
        async move {
            let mut context = HashMap::new();
            context.insert(
                "name".to_string(),
                TemplateValue::String("World".to_string()),
            );

            match template_engine.render("index.html", &context) {
                Ok(rendered) => res.body(rendered),
                Err(e) => {
                    res.set_status(500);
                    res.body(format!("Template rendering error: {}", e));
                }
            }
            Ok(())
        }
    });

    let router = Arc::new(router);

    server.use_middleware(move |req, res, next| {
        let router = Arc::clone(&router);
        Box::pin(async move { router.handle(req, res, next).await })
    });

    server.listen("127.0.0.1:7878");
}
```
