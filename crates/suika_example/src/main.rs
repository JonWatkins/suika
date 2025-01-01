use std::collections::HashMap;
use std::sync::Arc;

use suika::{
    server::{
        middleware::{
            combine_middlewares, cors_middleware, favicon_middleware, logger_middleware,
            static_file_middleware,
        },
        router::Router,
        Server,
    },
    templates::{TemplateEngine, TemplateValue},
};

pub fn main() {
    let server = Server::new();
    let mut router = Router::new();

    let template_engine = Arc::new({
        let mut engine = TemplateEngine::new();

        engine
            .load_templates_from_directory("crates/suika_example/templates")
            .expect("Failed to load templates from directory");

        engine
    });

    {
        let template_engine_hello = Arc::clone(&template_engine);
        router.get("/hello", move |_req, res, _next| {
            let template_engine = Arc::clone(&template_engine_hello);
            async move {
                let mut context = HashMap::new();
                context.insert(
                    "name".to_string(),
                    TemplateValue::String("World".to_string()),
                );

                match template_engine.render("crates/suika_example/templates/hello.html", &context)
                {
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
        let template_engine_user = Arc::clone(&template_engine);
        router.get("/user", move |_req, res, _next| {
            let template_engine = Arc::clone(&template_engine_user);
            async move {
                let mut user = HashMap::new();
                user.insert(
                    "name".to_string(),
                    TemplateValue::String("Alice".to_string()),
                );
                user.insert("age".to_string(), TemplateValue::String("30".to_string()));
                user.insert(
                    "email".to_string(),
                    TemplateValue::String("alice@example.com".to_string()),
                );

                let mut context = HashMap::new();
                context.insert("user".to_string(), TemplateValue::Object(user));

                match template_engine.render("user.html", &context) {
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
        Arc::new(favicon_middleware(
            "crates/suika_example/public/favicon.ico",
        )),
        Arc::new(static_file_middleware(
            "/public",
            "crates/suika_example/public",
            3600,
        )),
        Arc::new(logger_middleware),
        Arc::new(move |req, res, next| {
            let router = Arc::clone(&router);
            Box::pin(async move { router.handle(req, res, next).await })
        }),
    ]);

    server.use_middleware(move |req, res, next| combined_middleware(req, res, next));
    server.listen("127.0.0.1:7878");
}
