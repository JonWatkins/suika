# Rs-Serve

Work in progress simple web stack, this is not meant for production use. I still
need to clean up the API, write documentation and more tests ect. The API is
likely to change, as this is a very basic implementation and needs a ton more
work. currently the router only does simple string matching for example (this
will support regex in the future). 

Please dont use this version in production, as I have not checked for any bugs yet.
This is currently intended as a learning tool, and as such I am trying to minimise
the external crates im using.

## Example usage

```rust
use rs_serve::{
    router::Router,
    server::Server,
};

use std::sync::Arc;

fn main() {
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

### Post Data

```rust
use rs_serve::{
    router::Router,
    server::Server,
};

use std::sync::Arc;

fn main() {
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
use rs_serve::{
    router::Router,
    server::Server,
};

use std::sync::Arc;

fn main() {
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
use rs_serve::{
    middleware::{
        combine_middlewares, cors_middleware, favicon_middleware, logger_middleware,
        static_file_middleware,
    },
    router::Router,
    server::Server,
};

use std::sync::Arc;

fn main() {
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
        Arc::new(static_file_middleware("public", 3600)),
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
use rs_serve::{
    router::Router,
    server::Server,
    templates::{TemplateEngine, TemplateValue},
};

fn main() {
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
