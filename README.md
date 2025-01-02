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
use suika::server::{Router, Server};
use std::sync::Arc;

pub fn main() {
    let mut server = Server::new("127.0.0.1:8080");
    let mut router = Router::new("/");

    router.add_route(Some("GET"), "/", |_req, res| {
        Box::pin(async move {
            res.set_status(200).await;
            res.body("Hello World".to_string()).await;
            Ok(())
        })
    });


    server.use_middleware(Arc::new(router));
    server.run(None);
}
```

### Static file server

```rust
use suika::server::Server;
use suika::middleware::StaticFileMiddleware;
use std::sync::Arc;

pub fn main() {
    let mut server = Server::new("127.0.0.1:8080");
    server.use_middleware(Arc::new(StaticFileMiddleware::new("/public", "public", 3200)));
    server.run(None);
}
```

### Post Data

```rust
use suika::server::{Router, Server};
use std::sync::Arc;

pub fn main() {
    let mut server = Server::new("127.0.0.1:8080");
    let mut router = Router::new("/");

    router.add_route(Some("POST"), "/json", |req, res| {
        Box::pin(async move {
            if let Some(json_body) = req.json_body() {
                let response_message = format!("Data received: {:?}\n", json_body);
                res.set_status(200).await;
                res.body(response_message).await;
            } else {
                res.set_status(400).await;
                res.body("Invalid JSON data received!\n".to_string()).await;
            }
            Ok(())
        })
    });

    router.add_route(Some("POST"), "/form", |req, res| {
        Box::pin(async move {
            if let Some(form_data) = req.form_data() {
                let response_message = format!("Form Data received: {:?}\n", form_data);
                res.set_status(200).await;
                res.body(response_message).await;
            } else {
                res.set_status(400).await;
                res.body("Invalid form data received!\n".to_string()).await;
            }
            Ok(())
        })
    });

    server.use_middleware(Arc::new(router));
    server.run(None);
}
```

### Sending files

```rust
use suika::server::{Router, Server};
use std::sync::Arc;

pub fn main() {
    let mut server = Server::new("127.0.0.1:8080");
    let mut router = Router::new("/");

    router.add_route(Some("GET"), "/", |_req, res| {
        Box::pin(async move {
            if let Err(e) = res.send_file("index.html").await {
                res.error(e).await;
            }
            Ok(())
        })
    });

    server.use_middleware(Arc::new(router));
    server.run(None);
}
```

### Middleware

```rust
use std::sync::Arc;

use suika::{
    middleware::{
        CorsMiddleware, FaviconMiddleware, LoggerMiddleware, StaticFileMiddleware,
        WasmFileMiddleware,
    },
    server::{Router, Server},
};

pub fn main() {
    let mut server = Server::new("127.0.0.1:8080");
    let mut router = Router::new("/");

    router.add_route(Some("GET"), "/", |_req, res| {
        Box::pin(async move {
            res.set_status(200).await;
            res.body("Hello World".to_string()).await;
            Ok(())
        })
    });

    server.use_middleware(Arc::new(CorsMiddleware));
    server.use_middleware(Arc::new(LoggerMiddleware));

    server.use_middleware(Arc::new(FaviconMiddleware::new(
        "crates/suika_example/public/favicon.ico",
    )));

    server.use_middleware(Arc::new(StaticFileMiddleware::new(
        "/public", "crates/suika_example/public", 3600,
    )));

    server.use_middleware(Arc::new(WasmFileMiddleware::new("/wasm", 86400)));
    server.use_middleware(Arc::new(router));

    server.run(None);
}
```

### Template Engine

```rust
use std::collections::HashMap;
use std::sync::Arc;

use suika::{
    server::{Router, Server},
    templates::{TemplateEngine, TemplateValue},
};

pub fn main() {
    let mut server = Server::new("127.0.0.1:8080");
    let mut router = Router::new("/");

    let template_engine = Arc::new({
        let mut engine = TemplateEngine::new();

        engine
            .load_templates_from_directory("templates")
            .expect("Failed to load templates from directory");

        engine
    });

    {
        let template_engine = Arc::clone(&template_engine);
        router.add_route(Some("GET"), "/", move |_req, res| {
            let template_engine = Arc::clone(&template_engine);
            Box::pin(async move {
                let mut context = HashMap::new();

                context.insert(
                    "name".to_string(),
                    TemplateValue::String("World".to_string()),
                );

                match template_engine.render("hello.html", &context) {
                    Ok(rendered) => {
                        res.set_status(200).await;
                        res.body(rendered).await;
                    }
                    Err(_e) => {
                        res.set_status(500).await;
                        res.body("Template rendering error.".to_string()).await;
                    }
                }
                Ok(())
            })
        });
    }

    server.use_middleware(Arc::new(router));
    server.run(None);
}
```
