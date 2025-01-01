use rust_serve::{
    middleware::{
        combine_middlewares, cors_middleware, favicon_middleware, logger_middleware,
        static_file_middleware,
    },
    router::Router,
    server::Server,
    templates::{TemplateEngine, TemplateValue},
};

use std::collections::HashMap;
use std::sync::Arc;

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

    router.get("/", |_req, res, _next| async move {
        if let Err(e) = res.send_file("index.html") {
            eprintln!("Error: {}", e);
        }
        Ok(())
    });

    {
        let template_engine = Arc::clone(&template_engine);
        router.get("/hello", move |_req, res, _next| {
            let template_engine = Arc::clone(&template_engine);
            async move {
                let mut context = HashMap::new();
                context.insert(
                    "name".to_string(),
                    TemplateValue::String("World".to_string()),
                );

                match template_engine.render("hello.html", &context) {
                    Ok(rendered) => res.body(rendered),
                    Err(e) => {
                        res.set_status(500);
                        res.body(format!("Template rendering error: {}", e));
                    }
                }
                Ok(())
            }
        });
    }

    {
        let template_engine = Arc::clone(&template_engine);
        router.get("/conditional", move |_req, res, _next| {
            let template_engine = Arc::clone(&template_engine);
            async move {
                let mut context = HashMap::new();
                context.insert("is_member".to_string(), TemplateValue::Boolean(true));
                context.insert(
                    "name".to_string(),
                    TemplateValue::String("Alice".to_string()),
                );

                match template_engine.render("conditional.html", &context) {
                    Ok(rendered) => res.body(rendered),
                    Err(e) => {
                        res.set_status(500);
                        res.body(format!("Template rendering error: {}", e));
                    }
                }
                Ok(())
            }
        });
    }

    {
        let template_engine = Arc::clone(&template_engine);
        router.get("/loop", move |_req, res, _next| {
            let template_engine = Arc::clone(&template_engine);
            async move {
                let mut context = HashMap::new();
                context.insert(
                    "items".to_string(),
                    TemplateValue::Array(vec![
                        TemplateValue::String("One".to_string()),
                        TemplateValue::String("Two".to_string()),
                        TemplateValue::String("Three".to_string()),
                    ]),
                );

                match template_engine.render("loop.html", &context) {
                    Ok(rendered) => res.body(rendered),
                    Err(e) => {
                        res.set_status(500);
                        res.body(format!("Template rendering error: {}", e));
                    }
                }
                Ok(())
            }
        });
    }

    {
        let template_engine = Arc::clone(&template_engine);
        router.get("/extend", move |_req, res, _next| {
            let template_engine = Arc::clone(&template_engine);
            async move {
                match template_engine.render("child.html", &HashMap::new()) {
                    Ok(rendered) => res.body(rendered),
                    Err(e) => {
                        res.set_status(500);
                        res.body(format!("Template rendering error: {}", e));
                    }
                }
                Ok(())
            }
        });
    }

    {
        let template_engine = Arc::clone(&template_engine);
        router.get("/include", move |_req, res, _next| {
            let template_engine = Arc::clone(&template_engine);
            async move {
                match template_engine.render("include.html", &HashMap::new()) {
                    Ok(rendered) => res.body(rendered),
                    Err(e) => {
                        res.set_status(500);
                        res.body(format!("Template rendering error: {}", e));
                    }
                }
                Ok(())
            }
        });
    }

    router.post("/data", |req, res, _next| async move {
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
