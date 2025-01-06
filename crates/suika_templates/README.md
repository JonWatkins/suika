# Suika Templates

Suika Templates is a simple template engine for the Suika web stack, enabling
dynamic HTML generation.

**Note:** Suika is under active development and not intended for production use.
The API is subject to change and may lack comprehensive testing and
documentation.

## Features

- Parse and render templates with context values.
- Support for template directives such as variables, conditionals, loops,
  extends, includes, and blocks.
- Load templates from files or directories.

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
use suika_templates::{TemplateEngine, Context};

fn main() {
    let mut engine = TemplateEngine::new();
    engine.add_template("hello", "Hello, {{ name }}!");

    let mut context = Context::new();
    context.insert("name", "World"));

    let result = engine.render("hello", &context).expect("Failed to render template");
    assert_eq!(result, "Hello, World!");
}
```

## Loading Templates from Directory

You can also load multiple templates from a directory:

```rust
use suika_templates::{TemplateEngine, Context};

fn main() {
    // Create a new TemplateEngine
    let mut engine = TemplateEngine::new();
    let context = Context::new();

    // Load templates from the specified directory
    engine.load_templates("templates/**/*.html").expect("Failed to load templates");

    // Render the templates
    let result = engine.render("template1.html", &context).expect("Failed to render template");
    println!("Rendered Template 1:\n{}", result);

    let result = engine.render("template2.html", &context).expect("Failed to render template");
    println!("Rendered Template 2:\n{}", result);
}
```
