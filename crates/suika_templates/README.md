# Suika Templates: A Toy Template Engine

**Important: This is a personal toy project, developed as an experiment and learning exercise.**

**As a toy project, its future development is uncertain. It may or may not receive future updates, maintenance, or bug fixes. Please do not use it in production environments.**

Suika Templates is a simple template engine designed for the `Suika` web stack (also a toy project). It enables dynamic HTML generation with support for features like filters, inheritance, macros, and HTML minification.

The API is subject to change. This project is primarily for understanding template engine concepts and is not thoroughly tested or hardened for real-world applications.

## Features

-   Parse and render templates with context values
-   Support for template directives (variables, conditionals, loops)
-   Template inheritance with extends and blocks
-   Include other templates
-   Macro system for reusable components
-   Filter system for value transformation
-   Automatic HTML minification
-   Load templates from files or directories
-   Nested object access (e.g., user.name)

## Template Syntax

The `suika_templates` library supports the following syntax:

### Basic Features
-   **Comments**: `<%# This is a comment %>` (not rendered in output)
-   **Variables**: `<%= variable_name %>` or `<%= user.name %>`
-   **Filters**: `<%= name|upper %>` or `<%= items|length %>`

### Control Flow
-   **Conditionals**:
    -   Basic: `<% if condition %> ... <% elif condition %> ... <% else %> ... <% endif %>`
    -   Testing Functions:
        -   Defined: `<% if user is defined %> ... <% endif %>`
        -   Empty: `<% if array is empty %> ... <% endif %>`
        -   Value Compare: `<% if item is "value" %> ... <% endif %>`
        -   Number Type:
            -   `<% if num is odd %> ... <% endif %>`
            -   `<% if num is even %> ... <% endif %>`

-   **Loops**:
    -   Basic: `<% for item in items %> ... <% endfor %>`
    -   Loop Variables:
        -   `loop.index`: Zero-based iteration counter
        -   `loop.index1`: One-based iteration counter