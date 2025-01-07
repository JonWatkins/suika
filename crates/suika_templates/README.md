# Suika Templates

Suika Templates is a simple template engine for the Suika web stack, enabling
dynamic HTML generation with support for filters, inheritance, macros, and HTML minification.

**Note:** Suika is under active development and not intended for production use.
The API is subject to change.

## Features

- Parse and render templates with context values
- Support for template directives (variables, conditionals, loops)
- Template inheritance with extends and blocks
- Include other templates
- Macro system for reusable components
- Filter system for value transformation
- Automatic HTML minification
- Load templates from files or directories
- Nested object access (e.g., user.name)

## Template Syntax

The `suika_templates` library supports the following syntax:

## Template Syntax

### Basic Features
- **Comments**: `<%# This is a comment %>` (not rendered in output)
- **Variables**: `<%= variable_name %>` or `<%= user.name %>`
- **Filters**: `<%= name|upper %>` or `<%= items|length %>`

### Control Flow
- **Conditionals**: 
  - Basic: `<% if condition %> ... <% elif condition %> ... <% else %> ... <% endif %>`
  - Testing Functions:
    - Defined: `<% if user is defined %> ... <% endif %>`
    - Empty: `<% if array is empty %> ... <% endif %>`
    - Value Compare: `<% if item is "value" %> ... <% endif %>`
    - Number Type: 
      - `<% if num is odd %> ... <% endif %>`
      - `<% if num is even %> ... <% endif %>`

- **Loops**: 
  - Basic: `<% for item in items %> ... <% endfor %>`
  - Loop Variables:
    - `loop.index`: Zero-based iteration counter
    - `loop.index1`: One-based iteration counter
    - `loop.first`: True if first iteration
    - `loop.last`: True if last iteration
    - `loop.length`: Total number of items
  - Control:
    - Break: `<% break %>` (exit loop)
    - Continue: `<% continue %>` (skip to next iteration)

### Template Structure
- **Extends**: `<% extend base.html %>`
- **Includes**: `<% include header.html %>`
- **Blocks**: `<% block content %> ... <% endblock %>`

### Macros
- **Definition**: `<% macro name(param1, param2="default") %> ... <% endmacro %>`
- **Usage**: `<% call name(value1, value2) %>`

## Built-in Filters

String filters:
- `upper`: Convert text to uppercase
- `lower`: Convert text to lowercase
- `capitalize`: Capitalize first letter
- `trim`: Remove surrounding whitespace
- `reverse`: Reverse string characters

Collection filters:
- `length`: Get length of arrays or strings
- `first`: Get first element of array or string
- `last`: Get last element of array or string
- `join`: Join array elements with comma separator
- `json_encode`: Convert value to JSON string

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
