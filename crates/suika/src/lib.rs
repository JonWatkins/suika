//! # Suika
//!
//! `suika` is a web stack that re-exports features from various crates.
//! It provides a comprehensive set of tools for building web applications.

/// MIME type utilities.
///
/// This module re-exports all functionalities from the `suika_mime` crate.
///
/// # Examples
///
/// ```
/// use suika::mime::{get_mime_type, get_mime_type_from_path, MimeType};
///
/// let mime_type = get_mime_type("txt");
/// assert_eq!(mime_type, "text/plain".to_string());
/// 
/// let mime_type = get_mime_type_from_path("hello.txt");
/// assert_eq!(mime_type, "text/plain".to_string());
/// ```
///
/// ```
/// use suika::mime::MimeType;
///
/// let mime_type = MimeType::ApplicationJson;
/// assert_eq!(mime_type.as_str(), "application/json");
/// ```
pub mod mime {
  pub use suika_mime::{get_mime_type, get_mime_type_from_path, MimeType};
}

/// JSON handling utilities.
///
/// This module re-exports all functionalities from the `suika_json` crate.
///
/// # Examples
///
/// Parsing a JSON string:
///
/// ```
/// use suika::json::{parse_json, JsonValue};
///
/// let json = r#"{ "key": "value", "array": [1, 2, 3], "number": 123.45, "boolean": true, "null": null }"#;
/// let value = parse_json(json).unwrap();
///
/// assert_eq!(value, JsonValue::Object(vec![
///     ("key".to_string(), JsonValue::String("value".to_string())),
///     ("array".to_string(), JsonValue::Array(vec![
///         JsonValue::Number(1.0),
///         JsonValue::Number(2.0),
///         JsonValue::Number(3.0)
///     ])),
///     ("number".to_string(), JsonValue::Number(123.45)),
///     ("boolean".to_string(), JsonValue::Boolean(true)),
///     ("null".to_string(), JsonValue::Null)
/// ]));
/// ```
///
/// This library provides the `JsonValue` enum to represent JSON values and the `JsonParser`
/// struct to parse JSON strings into `JsonValue`.
pub mod json {
  pub use suika_json::{parse_json, JsonValue};
}

/// Utility functions.
///
/// This module re-exports all functionalities from the `suika_utils` crate.
///
/// # Examples
///
/// ```
/// use suika::utils::parse_query_string;
/// let query = "name=John&age=30";
/// let params = parse_query_string(query);
/// assert_eq!(params.get("name"), Some(&"John".to_string()));
/// assert_eq!(params.get("age"), Some(&"30".to_string()));
/// ```
///
/// ```
/// use suika::utils::skip_whitespace;
/// let input = "   abc";
/// let mut chars = input.chars();
/// let mut current_char = chars.next();
/// skip_whitespace(&mut chars, &mut current_char);
/// assert_eq!(current_char, Some('a'));
/// ```
///
/// ```
/// use suika::utils::expect_sequence;
/// let input = "true";
/// let mut chars = input.chars();
/// let mut current_char = chars.next();
/// assert!(expect_sequence(&mut chars, &mut current_char, "true").is_ok());
/// assert_eq!(current_char, None);
///
/// let input = "tru";
/// let mut chars = input.chars();
/// let mut current_char = chars.next();
/// assert!(expect_sequence(&mut chars, &mut current_char, "true").is_err());
/// ```
///
/// ```
/// use suika::utils::build_url;
/// let base = "https://example.com";
/// let mut params = std::collections::HashMap::new();
/// params.insert("name", "John");
/// params.insert("age", "30");
/// let url = build_url(base, &params);
/// assert_eq!(url, "https://example.com?age=30&name=John");
/// ```
///
/// ```
/// use suika::utils::parse_url;
/// let url = "https://example.com/path?name=John&age=30";
/// let components = parse_url(url).unwrap();
/// assert_eq!(components.0, "https");
/// assert_eq!(components.1, "example.com");
/// assert_eq!(components.2, "/path");
/// assert_eq!(components.3.get("name"), Some(&"John".to_string()));
/// assert_eq!(components.3.get("age"), Some(&"30".to_string()));
/// ```
///
/// ```
/// use suika::utils::noop_waker;
/// use std::task::{Context, Poll};
/// use std::future::Future;
/// use std::pin::Pin;
/// use std::sync::{
///     atomic::{AtomicBool, Ordering},
///     Arc,
/// };
///
/// let waker = noop_waker();
/// let mut cx = Context::from_waker(&waker);
///
/// let ready = Arc::new(AtomicBool::new(false));
/// let ready_clone = Arc::clone(&ready);
///
/// let mut future = Box::pin(async move {
///     ready_clone.store(true, Ordering::SeqCst);
/// });
///
/// assert!(!ready.load(Ordering::SeqCst));
/// let _ = future.as_mut().poll(&mut cx);
/// assert!(ready.load(Ordering::SeqCst));
/// ```
pub mod utils {
  pub use suika_utils::*;
}

/// Template rendering utilities.
///
/// This module re-exports all functionalities from the `suika_templates` crate.
///
/// # Examples
///
/// ```rust,ignore
/// use suika::templates::{TemplateEngine, TemplateValue};
/// use std::collections::HashMap;
///
/// let mut engine = TemplateEngine::new();
/// engine.add_template("hello", "Hello, {{ name }}!");
///
/// let mut context = HashMap::new();
/// context.insert("name".to_string(), TemplateValue::String("World".to_string()));
///
/// let result = engine.render("hello", &context).expect("Failed to render template");
/// assert_eq!(result, "Hello, World!");
/// ```
///
/// ### Loading Templates from Directory
///
/// ```rust,ignore
/// use suika::templates::TemplateEngine;
/// use std::collections::HashMap;
///
/// // Assuming the templates directory contains template1.html and template2.html
/// let mut engine = TemplateEngine::new();
/// engine.load_templates_from_directory("templates").expect("Failed to load templates");
///
/// let result = engine.render("template1.html", &HashMap::new()).expect("Failed to render template");
/// println!("Rendered Template 1:\n{}", result);
///
/// let result = engine.render("template2.html", &HashMap::new()).expect("Failed to render template");
/// println!("Rendered Template 2:\n{}", result);
/// ```
pub mod templates {
  pub use suika_templates::{
      TemplateEngine, TemplateParser, TemplateToken, TemplateValue
  };
}

/// Server functionalities.
///
/// This module re-exports all functionalities from the `suika_server` crate.
///
/// # Examples
///
/// ```rust,ignore
/// use suika::server::Server;
///
/// let server = Server::new();
/// server.listen("127.0.0.1:7878");
/// ```
pub mod server {
  pub use suika_server::*;
}

/// WebAssembly middleware utilities.
///
/// This module re-exports all functionalities from the `suika_wasm` crate.
///
/// # Examples
///
/// Serving WebAssembly files:
///
/// ```rust,ignore
/// use suika::wasm::wasm_file_middleware;
/// use suika::server::Server;
/// use std::sync::Arc;
///
/// let server = Server::new();
/// let middleware = wasm_file_middleware("/wasm", "public/wasm", 3600);
/// server.use_middleware(Arc::new(middleware));
/// server.listen("127.0.0.1:7878");
/// ```
pub mod wasm {
  pub use suika_wasm::*;
}
