# Suika Templates

A simple and lightweight template engine library for Rust that can parse and
render templates with various directives.

## Features

- Parse and render templates with context values.
- Support for template directives such as variables, conditionals, loops,
  extends, includes, and blocks.
- Load templates from files or directories.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
suika_templates = "0.1.0"
```

## Template Directives

The `suika_templates` library supports various directives within templates:

- **Variables**: `{{ variable_name }}`
- **Conditionals**: `{% if condition %} ... {% else %} ... {% endif %}`
- **Loops**: `{% for item in items %} ... {% endfor %}`
- **Extends**: `{% extend "base.html" %}`
- **Includes**: `{% include "header.html" %}`
- **Blocks**: `{% block content %} ... {% endblock %}`

## Usage

Here's an example of how to use `suika_templates` to parse and render a
template:

```rust
use suika_templates::{TemplateEngine, TemplateValue};
use std::collections::HashMap;

fn main() {
    let mut engine = TemplateEngine::new();
    engine.add_template("hello", "Hello, {{ name }}!");

    let mut context = HashMap::new();
    context.insert("name".to_string(), TemplateValue::String("World".to_string()));

    let result = engine.render("hello", &context).expect("Failed to render template");
    assert_eq!(result, "Hello, World!");
}
```

## Loading Templates from Directory

You can also load multiple templates from a directory:

```rust
use suika_templates::TemplateEngine;
use std::collections::HashMap;

fn main() {
    // Create a new TemplateEngine
    let mut engine = TemplateEngine::new();

    // Load templates from the specified directory
    engine.load_templates_from_directory("templates").expect("Failed to load templates");

    // Render the templates
    let result = engine.render("template1.html", &HashMap::new()).expect("Failed to render template");
    println!("Rendered Template 1:\n{}", result);

    let result = engine.render("template2.html", &HashMap::new()).expect("Failed to render template");
    println!("Rendered Template 2:\n{}", result);
}
```
