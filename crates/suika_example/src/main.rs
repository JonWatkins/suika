mod todos;

use crate::todos::TodoStore;
use std::sync::Arc;

use suika::{
    json::JsonValue,
    middleware::{
        CorsMiddleware, FaviconMiddleware, LoggerMiddleware, StaticFileMiddleware,
        WasmFileMiddleware,
    },
    server::{Router, Server},
    templates::{Context, TemplateEngine},
};

fn main() {
    let mut server = Server::new("127.0.0.1:8080");
    let mut main_router = Router::new("/");
    let todo_store = TodoStore::new();

    let template_engine = {
        let mut engine = TemplateEngine::new();

        engine
            .load_templates_from_directory("crates/suika_example/templates")
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
            let json = JsonValue::Object(vec![
                (
                    "name".to_string(),
                    JsonValue::String("John Doe".to_string()),
                ),
                ("age".to_string(), JsonValue::Number(30.0)),
                ("is_student".to_string(), JsonValue::Boolean(false)),
                (
                    "address".to_string(),
                    JsonValue::Object(vec![
                        (
                            "street".to_string(),
                            JsonValue::String("123 Main St".to_string()),
                        ),
                        ("city".to_string(), JsonValue::String("Anytown".to_string())),
                        ("zip".to_string(), JsonValue::String("12345".to_string())),
                    ]),
                ),
                (
                    "courses".to_string(),
                    JsonValue::Array(vec![
                        JsonValue::String("Math".to_string()),
                        JsonValue::String("Science".to_string()),
                    ]),
                ),
            ]);

            res.set_status(200).await;
            res.body_json(json).await;

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

    // main_router.get("/user", |_req, res| {
    //     Box::pin(async move {
    //         let mut user = HashMap::new();

    //         user.insert("name", "Alice");
    //         user.insert("age", "30");
    //         user.insert("email", "alice@example.com");

    //         let mut context = Context::new();
    //         context.insert("user", JsonValue::Object(user));

    //         res.set_status(200).await;
    //         res.render_template("user.html", &context).await?;

    //         Ok(())
    //     })
    // });

    main_router.get(r"/items/(?P<id>\d+)$", |req, res| {
        Box::pin(async move {
            res.set_status(200).await;
            let item_id = req.param("id").map(|s| s.to_string()).unwrap_or_default();
            res.body(format!("You requested item with ID: {}", item_id))
                .await;
            Ok(())
        })
    });

    let mut ui_router = Router::new("/ui");

    ui_router.get(r"/?$", |_req, res| {
        Box::pin(async move {
            let context = Context::new();
            res.set_status(200).await;
            res.render_template("ui.html", &context).await?;
            Ok(())
        })
    });

    // ui_router.get("/todos", |req, res| {
    //     Box::pin(async move {
    //         let mut context = HashMap::new();

    //         if let Some(store) = req.module::<TodoStore>("todo_store") {
    //             let todos = store.get_todos();

    //             context.insert(
    //                 "todos".to_string(),
    //                 TemplateValue::Array(
    //                     todos
    //                         .iter()
    //                         .map(|todo| {
    //                             let mut todo_map = HashMap::new();
    //                             todo_map.insert(
    //                                 "id".to_string(),
    //                                 TemplateValue::String(todo.id.to_string()),
    //                             );
    //                             todo_map.insert(
    //                                 "title".to_string(),
    //                                 TemplateValue::String(todo.title.clone()),
    //                             );
    //                             todo_map.insert(
    //                                 "slug".to_string(),
    //                                 TemplateValue::String(todo.slug.clone()),
    //                             );
    //                             todo_map.insert(
    //                                 "content".to_string(),
    //                                 TemplateValue::String(todo.content.clone()),
    //                             );
    //                             TemplateValue::Object(todo_map)
    //                         })
    //                         .collect(),
    //                 ),
    //             );

    //             res.set_status(200).await;
    //             res.header("Content-Type", "text/html").await;
    //             res.render_template("ui_todos.html", &context).await?;
    //         } else {
    //             res.set_status(404).await;
    //             res.body("No todos found".to_string()).await;
    //         }

    //         Ok(())
    //     })
    // });

    ui_router.post("/add_post", move |_req, _res| {
        Box::pin(async move { Ok(()) })
    });

    main_router.mount(ui_router);

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

    server.use_middleware(Arc::new(WasmFileMiddleware::new("/wasm", 86400)));
    server.use_middleware(Arc::new(main_router));

    server.run(None);
}
