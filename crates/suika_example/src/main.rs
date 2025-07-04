mod todos;
mod user;

use crate::todos::TodoStore;
use crate::user::User;
use std::sync::Arc;

use suika::{
    macros::json,
    middleware::{CorsMiddleware, FaviconMiddleware, LoggerMiddleware, StaticFileMiddleware},
    server::{Router, Server},
    templates::{Context, TemplateEngine},
};

fn main() {
    let mut server = Server::new("127.0.0.1:1337");
    let mut main_router = Router::new("/");
    let todo_store = TodoStore::new();

    let template_engine = {
        let mut engine = TemplateEngine::new();

        engine.register_filter("reverse", |value: String| {
            Ok(value.chars().rev().collect::<String>())
        });

        engine
            .load_templates("crates/suika_example/templates/**/*.html")
            .expect("Failed to load templates from directory");

        engine
    };

    todo_store.add_todo(
        "First Todo".to_string(),
        "This is the content of the first todo.".to_string(),
    );

    todo_store.add_todo(
        "Second Todo".to_string(),
        "This is the content of the second todo.".to_string(),
    );

    server.use_templates(template_engine);
    server.use_module("todo_store", todo_store);

    main_router.get(r"/$", |_req, res| {
        Box::pin(async move {
            if let Err(e) = res.send_file("crates/suika_example/index.html").await {
                res.error(e).await;
            }
            Ok(())
        })
    });

    main_router.get("/todos", |req, res| {
        Box::pin(async move {
            if let Some(store) = req.module::<TodoStore>("todo_store") {
                let todos = store.to_json();
                let mut context = Context::new();
                context.insert("todos", todos);

                res.set_status(200).await;
                res.header("Content-Type", "text/html").await;
                res.render_template("todos/list.html", &context).await?;
            } else {
                res.set_status(404).await;
                res.body("No todos found".to_string()).await;
            }

            Ok(())
        })
    });

    main_router.get("/todo_json", |req, res| {
        Box::pin(async move {
            if let Some(store) = req.module::<TodoStore>("todo_store") {
                let todos = store.to_json();

                res.set_status(200).await;
                res.body_json(todos).await;
            } else {
                res.set_status(404).await;
                res.body("No todos found".to_string()).await;
            }
            Ok(())
        })
    });

    main_router.get("json", |_req, res| {
        Box::pin(async move {
            let user = json!({
                "name" => "John Doe",
                "age" => 30,
                "is_student" => false,
                "email" => None::<String>,
                "address" => json!({
                    "street" => "123 Main St",
                    "city" => "Anytown",
                    "zip" => "12345",
                }),
                "courses" => vec!["Math", "Science"],
            });

            res.set_status(200).await;
            res.body_json(user).await;

            Ok(())
        })
    });

    main_router.get("/hello", |_req, res| {
        Box::pin(async move {
            let mut context = Context::new();
            context.insert("name", "World");

            res.set_status(200).await;
            res.render_template("hello.html", &context).await?;

            Ok(())
        })
    });

    main_router.get("/include", |_req, res| {
        Box::pin(async move {
            let mut context = Context::new();
            context.insert("name", "World");

            res.set_status(200).await;
            res.render_template("include.html", &context).await?;

            Ok(())
        })
    });

    main_router.get("/conditional", |_req, res| {
        Box::pin(async move {
            let mut context = Context::new();

            context.insert("is_member", true);
            context.insert("name", "Bob");

            res.set_status(200).await;
            res.render_template("conditional.html", &context).await?;

            Ok(())
        })
    });

    main_router.get("/loop", |_req, res| {
        Box::pin(async move {
            let mut context = Context::new();
            context.insert("items", vec!["One", "Two", "Three"]);

            res.set_status(200).await;
            res.render_template("loop.html", &context).await?;

            Ok(())
        })
    });

    main_router.get("/user", |_req, res| {
        Box::pin(async move {
            let user = User {
                name: "Alice".to_string(),
                age: 30,
                is_student: false,
                email: Some("alice@example.com".to_string()),
                address: None,
                courses: vec![],
            };

            let mut context = Context::new();
            context.insert("user", user);

            res.set_status(200).await;
            res.render_template("user.html", &context).await?;

            Ok(())
        })
    });

    main_router.get(r"/items/(?P<id>\d+)$", |req, res| {
        Box::pin(async move {
            res.set_status(200).await;
            let item_id = req.param("id").map(|s| s.to_string()).unwrap_or_default();
            res.body(format!("You requested item with ID: {}", item_id))
                .await;
            Ok(())
        })
    });

    server.use_middleware(Arc::new(CorsMiddleware));
    server.use_middleware(Arc::new(LoggerMiddleware));

    server.use_middleware(Arc::new(FaviconMiddleware::new(
        "crates/suika_example/public/favicon.ico",
    )));

    server.use_middleware(Arc::new(StaticFileMiddleware::new(
        "/public",
        "crates/suika_example/public",
        3600,
    )));

    server.use_middleware(Arc::new(main_router));

    server.run(None);
}
