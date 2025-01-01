use std::{collections::HashMap, sync::Arc};
use suika_templates::{TemplateEngine, TemplateValue};

use suika_server::{
    middleware::{
        combine_middlewares, cors_middleware, favicon_middleware, logger_middleware,
        static_file_middleware,
    },
    router::Router,
    Server,
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
