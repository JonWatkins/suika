//! # Template Engine
//!
//! A simple template engine library for Rust that can parse and render templates with various directives.
//!
//! ## Example
//!
//! ```rust,ignore
//! use suika_templates::{TemplateEngine, TemplateValue};
//! use std::collections::HashMap;
//!
//! let mut engine = TemplateEngine::new();
//! engine.add_template("hello", "Hello, {{ name }}!");
//!
//! let mut context = HashMap::new();
//! context.insert("name".to_string(), TemplateValue::String("World".to_string()));
//!
//! let result = engine.render("hello", &context).expect("Failed to render template");
//! assert_eq!(result, "Hello, World!");
//! ```
//!
//! ### Loading Templates from Directory
//!
//! ```rust,ignore
//! use suika_templates::TemplateEngine;
//! use std::collections::HashMap;
//!
//! // Assuming the templates directory contains template1.html and template2.html
//! let mut engine = TemplateEngine::new();
//! engine.load_templates_from_directory("templates").expect("Failed to load templates");
//!
//! let result = engine.render("template1.html", &HashMap::new()).expect("Failed to render template");
//! println!("Rendered Template 1:\n{}", result);
//!
//! let result = engine.render("template2.html", &HashMap::new()).expect("Failed to render template");
//! println!("Rendered Template 2:\n{}", result);
//! ```

pub mod context;
pub mod parser;
pub mod template_engine;
pub mod template_token;
pub mod template_value;

pub use parser::TemplateParser;
pub use template_engine::TemplateEngine;
pub use template_token::TemplateToken;
pub use template_value::TemplateValue;
