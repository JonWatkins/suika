use crate::context::Context;
use crate::filters::{FilterRegistry, FromJsonValue, IntoJsonValue};
use crate::TemplateParser;
use crate::TemplateToken;
use glob::glob;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use suika_json::JsonValue;
use suika_utils::minify_html;

#[derive(Debug, Clone)]
pub struct TemplateEngine {
    templates: HashMap<String, String>,
    filters: FilterRegistry,
    macros: Arc<Mutex<HashMap<String, (String, Vec<String>, Vec<TemplateToken>)>>>,
    included_templates: Arc<Mutex<HashSet<String>>>,
}

impl TemplateEngine {
    /// Creates a new `TemplateEngine`.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_templates::template_engine::TemplateEngine;
    ///
    /// let engine = TemplateEngine::new();
    /// ```
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
            filters: FilterRegistry::new(),
            macros: Arc::new(Mutex::new(HashMap::new())),
            included_templates: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Adds a template with the given name and content.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the template.
    /// * `content` - The content of the template.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_templates::template_engine::TemplateEngine;
    ///
    /// let mut engine = TemplateEngine::new();
    /// engine.add_template("hello", "Hello, {{ name }}!");
    /// ```
    pub fn add_template(&mut self, name: &str, content: &str) {
        let mut parser = TemplateParser::new(content);
        if let Ok(tokens) = parser.parse() {
            // Process macros first
            for (i, token) in tokens.iter().enumerate() {
                if let TemplateToken::MacroDefinition(macro_name, params) = token {
                    let mut macro_tokens = Vec::new();
                    let mut j = i + 1;
                    while j < tokens.len() {
                        if let TemplateToken::EndMacro = tokens[j] {
                            break;
                        }
                        macro_tokens.push(tokens[j].clone());
                        j += 1;
                    }
                    self.macros.lock().unwrap().insert(
                        macro_name.clone(),
                        (name.to_string(), params.clone(), macro_tokens),
                    );
                }
            }
        }
        self.templates.insert(name.to_string(), content.to_string());
    }

    /// ```
    /// use suika_templates::template_engine::TemplateEngine;
    /// use suika_templates::context::Context;
    /// use std::fs;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// // Setup: Create a temporary directory and files for the test
    /// let temp_dir = "temp_templates";
    /// let nested_dir = format!("{}/nested", temp_dir);
    ///
    /// fs::create_dir_all(&nested_dir).expect("Failed to create temp directory");
    /// let mut file = File::create(format!("{}/template1.html", temp_dir)).expect("Failed to create file");
    /// writeln!(file, "<html><body>Template 1</body></html>").expect("Failed to write to file");
    /// let mut file = File::create(format!("{}/template2.html", &nested_dir)).expect("Failed to create file");
    /// writeln!(file, "<html><body>Template 2</body></html>").expect("Failed to write to file");
    ///
    /// // Test: Load templates
    /// let mut engine = TemplateEngine::new();
    /// engine.load_templates("temp_templates/**/*.html").expect("Failed to load templates");
    ///
    /// // Verify: Render templates and check results
    /// let result = engine.render("template1.html", &Context::new()).expect("Failed to render template");
    /// assert_eq!(result, "<html><body>Template 1</body></html>");
    /// let result = engine.render("nested/template2.html", &Context::new()).expect("Failed to render template");
    /// assert_eq!(result, "<html><body>Template 2</body></html>");
    ///
    /// // Teardown: Remove the temporary directory and files
    /// fs::remove_dir_all(temp_dir).expect("Failed to remove temp directory");
    /// ```
    pub fn load_templates(&mut self, pattern: &str) -> Result<(), String> {
        let base_dir = Path::new(pattern)
            .parent()
            .and_then(Path::parent)
            .ok_or("Invalid base directory in pattern")?;

        if !base_dir.exists() {
            return Err("Base directory does not exist".to_string());
        }

        if !base_dir.is_dir() {
            return Err("Base directory is not a directory".to_string());
        }

        for entry in glob(pattern).map_err(|e| e.to_string())? {
            match entry {
                Ok(path) => {
                    if path.is_file() {
                        if let Some(template_name) = self.construct_template_name(base_dir, &path) {
                            let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
                            self.add_template(&template_name, content.trim());
                        }
                    }
                }
                Err(e) => return Err(e.to_string()),
            }
        }
        Ok(())
    }

    /// Constructs a template name based on the file path, stripping the base directory.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - The base directory to strip from the template path.
    /// * `path` - The path of the template file.
    ///
    /// # Returns
    ///
    /// Returns a string with the constructed template name.
    fn construct_template_name(&self, base_dir: &Path, path: &Path) -> Option<String> {
        path.strip_prefix(base_dir)
            .ok()?
            .to_str()
            .map(|s| s.to_string())
    }

    /// Renders a template with the given name and context.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the template to render.
    /// * `context` - A `Context` containing the context values to use in the template.
    ///
    /// # Errors
    ///
    /// Returns an error if the template is not found or if there is an issue processing the template.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_templates::template_engine::TemplateEngine;
    /// use suika_templates::context::Context;
    ///
    /// let mut engine = TemplateEngine::new();
    /// engine.add_template("hello", "Hello, <%= name %>!");
    ///
    /// let mut context = Context::new();
    /// context.insert("name", "World");
    ///
    /// let result = engine.render("hello", &context).expect("Failed to render template");
    /// assert_eq!(result, "Hello, World!");
    /// ```
    pub fn render(&self, name: &str, context: &Context) -> Result<String, String> {
        let template = self
            .templates
            .get(name)
            .ok_or_else(|| format!("Template '{}' not found", name))?;
        let mut parser = TemplateParser::new(template);
        let mut tokens = parser.parse()?;

        tokens = self.handle_extends(&tokens)?;

        let output = self.process_tokens(&tokens, context)?;
        let minified_output = minify_html(&output);
        Ok(minified_output)
    }

    fn handle_extends(&self, tokens: &[TemplateToken]) -> Result<Vec<TemplateToken>, String> {
        let mut result_tokens = Vec::new();
        let mut blocks = HashMap::new();
        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                TemplateToken::Extend(parent_template) => {
                    let parent_tokens = self.get_template_tokens(parent_template)?;
                    result_tokens.extend(parent_tokens);
                }
                TemplateToken::Block(block_name) => {
                    let mut block_tokens = Vec::new();
                    i += 1;
                    while i < tokens.len() {
                        if let TemplateToken::EndBlock = &tokens[i] {
                            break;
                        }
                        block_tokens.push(tokens[i].clone());
                        i += 1;
                    }
                    blocks.insert(block_name.clone(), block_tokens);
                }
                _ => result_tokens.push(tokens[i].clone()),
            }
            i += 1;
        }

        let mut final_tokens = Vec::new();
        i = 0;
        while i < result_tokens.len() {
            match &result_tokens[i] {
                TemplateToken::Block(block_name) => {
                    if let Some(block_tokens) = blocks.get(block_name) {
                        final_tokens.extend(block_tokens.clone());
                    }
                    while i < result_tokens.len() {
                        if let TemplateToken::EndBlock = &result_tokens[i] {
                            break;
                        }
                        i += 1;
                    }
                }
                _ => final_tokens.push(result_tokens[i].clone()),
            }
            i += 1;
        }

        Ok(final_tokens)
    }

    fn get_template_tokens(&self, name: &str) -> Result<Vec<TemplateToken>, String> {
        let template = self
            .templates
            .get(name)
            .ok_or_else(|| format!("Template '{}' not found", name))?;
        let mut parser = TemplateParser::new(template);
        parser.parse()
    }

    fn process_tokens(
        &self,
        tokens: &[TemplateToken],
        context: &Context,
    ) -> Result<String, String> {
        let mut output = String::new();
        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                TemplateToken::Text(text) => self.process_text(text, &mut output),
                TemplateToken::Variable(name, filters) => {
                    self.process_variable(name, filters, context, &mut output)?
                }
                TemplateToken::If(condition) => {
                    i = self.process_if(condition, tokens, context, &mut output, i)?
                }
                TemplateToken::For(var, array) => {
                    i = self.process_for(var, array, tokens, context, &mut output, i)?
                }
                TemplateToken::Include(template_name) => {
                    self.process_include(template_name, context, &mut output)?
                }
                TemplateToken::MacroCall(name, args) => {
                    self.process_macro_call(name, args, context, &mut output)?
                }
                TemplateToken::MacroDefinition(_, _) => {
                    // Skip over macro definition tokens until we find EndMacro
                    while i < tokens.len() {
                        if let TemplateToken::EndMacro = tokens[i] {
                            break;
                        }
                        i += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        Ok(output)
    }

    fn process_text(&self, text: &str, output: &mut String) {
        output.push_str(text);
    }

    fn process_variable(
        &self,
        name: &str,
        filters: &[String],
        context: &Context,
        output: &mut String,
    ) -> Result<(), String> {
        if let Some(value_str) = self.resolve_variable(name, context) {
            let mut value = match context.get(name) {
                Some(v) => v.clone(),
                None => JsonValue::String(value_str),
            };

            // Apply filters in order
            for filter_name in filters {
                if let Some(filter) = self.filters.get(filter_name) {
                    value = filter(value, vec![])?;
                } else {
                    return Err(format!("Filter '{}' not found", filter_name));
                }
            }

            // Convert the final value to a string without quotes
            match value {
                JsonValue::String(s) => output.push_str(&s),
                JsonValue::Number(n) => output.push_str(&n.to_string()),
                JsonValue::Array(arr) => output.push_str(&arr.len().to_string()),
                _ => output.push_str(&value.to_string()),
            }
            Ok(())
        } else {
            Err(format!("Variable '{}' not found in context", name))
        }
    }

    fn resolve_variable(&self, name: &str, context: &Context) -> Option<String> {
        let parts: Vec<&str> = name.split('.').collect();
        let mut current_value = context.get(parts[0])?;

        for part in &parts[1..] {
            match current_value {
                JsonValue::Object(map) => {
                    current_value = &map.iter().find(|(k, _)| k == part)?.1;
                }
                _ => return None,
            }
        }

        match current_value {
            JsonValue::String(s) => Some(s.clone()),
            JsonValue::Boolean(b) => Some(b.to_string()),
            JsonValue::Array(arr) => Some(format!(
                "[{}]",
                arr.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )),
            JsonValue::Object(obj) => Some(format!(
                "{{{}}}",
                obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ")
            )),
            JsonValue::Number(n) => Some(n.to_string()),
            JsonValue::Null => Some("null".to_string()),
        }
    }

    fn process_if(
        &self,
        condition: &str,
        tokens: &[TemplateToken],
        context: &Context,
        output: &mut String,
        mut i: usize,
    ) -> Result<usize, String> {
        let mut if_tokens = Vec::new();
        let mut else_tokens = Vec::new();
        let mut in_else = false;
        i += 1;
        while i < tokens.len() {
            match &tokens[i] {
                TemplateToken::EndIf => break,
                TemplateToken::Else => in_else = true,
                _ => {
                    if in_else {
                        else_tokens.push(tokens[i].clone());
                    } else {
                        if_tokens.push(tokens[i].clone());
                    }
                }
            }
            i += 1;
        }
        if let Some(JsonValue::Boolean(true)) = context.get(condition) {
            output.push_str(&self.process_tokens(&if_tokens, context)?);
        } else {
            output.push_str(&self.process_tokens(&else_tokens, context)?);
        }
        Ok(i)
    }

    fn process_for(
        &self,
        var: &str,
        array: &str,
        tokens: &[TemplateToken],
        context: &Context,
        output: &mut String,
        mut i: usize,
    ) -> Result<usize, String> {
        if let Some(JsonValue::Array(values)) = context.get(array) {
            let mut for_tokens = Vec::new();
            i += 1;
            while i < tokens.len() {
                if let TemplateToken::EndFor = &tokens[i] {
                    break;
                }
                for_tokens.push(tokens[i].clone());
                i += 1;
            }
            for value in values {
                let mut loop_context = context.clone();
                loop_context.insert(var, value.clone());
                output.push_str(&self.process_tokens(&for_tokens, &loop_context)?);
            }
        } else {
            while i < tokens.len() {
                if let TemplateToken::EndFor = &tokens[i] {
                    break;
                }
                i += 1;
            }
        }
        Ok(i)
    }

    fn process_include(
        &self,
        template_name: &str,
        context: &Context,
        output: &mut String,
    ) -> Result<(), String> {
        self.included_templates
            .lock()
            .unwrap()
            .insert(template_name.to_string());

        let include_tokens = self.get_template_tokens(template_name)?;

        // First pass: process macro definitions
        for (i, token) in include_tokens.iter().enumerate() {
            if let TemplateToken::MacroDefinition(macro_name, params) = token {
                let mut macro_tokens = Vec::new();
                let mut j = i + 1;
                while j < include_tokens.len() {
                    if let TemplateToken::EndMacro = include_tokens[j] {
                        break;
                    }
                    macro_tokens.push(include_tokens[j].clone());
                    j += 1;
                }
                self.macros.lock().unwrap().insert(
                    macro_name.clone(),
                    (template_name.to_string(), params.clone(), macro_tokens),
                );
            }
        }

        // Second pass: process the rest of the template
        let result = self.process_tokens(&include_tokens, context)?;
        output.push_str(&result);
        Ok(())
    }

    pub fn register_filter<F, T, R>(&mut self, name: &str, filter: F)
    where
        F: Fn(T) -> Result<R, String> + Send + Sync + 'static,
        T: FromJsonValue,
        R: IntoJsonValue,
    {
        self.filters.register(name, filter);
    }

    fn process_macro_call(
        &self,
        name: &str,
        args: &[String],
        context: &Context,
        output: &mut String,
    ) -> Result<(), String> {
        // First find the macro definition
        if let Some((template_name, params, tokens)) =
            self.macros.lock().unwrap().get(name).cloned()
        {
            // Check if the template containing this macro has been included
            if !self
                .included_templates
                .lock()
                .unwrap()
                .contains(&template_name)
            {
                return Err(format!(
                    "Macro '{}' cannot be used without including template '{}'",
                    name, template_name
                ));
            }

            let mut macro_context = Context::new();

            for (i, param) in params.iter().enumerate() {
                let (param_name, default_value) = if param.contains('=') {
                    let parts: Vec<&str> = param.split('=').collect();
                    (parts[0].trim(), Some(parts[1].trim().trim_matches('"')))
                } else {
                    (param.trim(), None)
                };

                let value = if i < args.len() {
                    // Remove quotes from string literals
                    let arg = args[i].trim_matches('"');
                    // Try to get from context first
                    if let Some(ctx_value) = context.get(arg) {
                        ctx_value.clone()
                    } else {
                        // Use as literal string if not in context
                        JsonValue::String(arg.to_string())
                    }
                } else if let Some(default) = default_value {
                    JsonValue::String(default.to_string())
                } else {
                    return Err(format!("Missing argument for parameter '{}'", param_name));
                };

                macro_context.insert(param_name, value);
            }

            let result = self.process_tokens(&tokens, &macro_context)?;
            output.push_str(&result.trim());
            Ok(())
        } else {
            Err(format!("Macro '{}' not found", name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::Write;

    // Define a sample Product struct to test the rendering with structs
    #[derive(Debug, Clone)]
    struct Product {
        name: String,
    }

    impl From<Product> for JsonValue {
        fn from(product: Product) -> Self {
            JsonValue::Object(vec![("name".to_string(), JsonValue::String(product.name))])
        }
    }

    #[test]
    fn test_render_variable() {
        let mut engine = TemplateEngine::new();
        engine.add_template("hello", "Hello, <%= name %>!");

        let mut context = Context::new();
        context.insert("name", "World");

        let result = engine
            .render("hello", &context)
            .expect("Failed to render template");
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_render_struct() {
        let mut engine = TemplateEngine::new();
        engine.add_template("product", "Product: <%= product.name %>");

        let product = Product {
            name: "Widget".to_string(),
        };

        let mut context = Context::new();
        context.insert("product", JsonValue::from(product));

        let result = engine
            .render("product", &context)
            .expect("Failed to render template");
        assert_eq!(result, "Product: Widget");
    }

    #[test]
    fn test_render_conditional_true() {
        let mut engine = TemplateEngine::new();
        engine.add_template(
            "conditional",
            "<% if is_member %>Welcome, <%= name %>!<% endif %>",
        );

        let mut context = Context::new();
        context.insert("is_member", true);
        context.insert("name", "Alice");

        let result = engine
            .render("conditional", &context)
            .expect("Failed to render template");
        assert_eq!(result, "Welcome, Alice!");
    }

    #[test]
    fn test_render_conditional_false() {
        let mut engine = TemplateEngine::new();
        engine.add_template(
            "conditional",
            "<% if is_member %>Welcome, <%= name %>!<% endif %>",
        );

        let mut context = Context::new();
        context.insert("is_member", false);
        context.insert("name", "Alice");

        let result = engine
            .render("conditional", &context)
            .expect("Failed to render template");
        assert_eq!(result, "");
    }

    #[test]
    fn test_render_if_else_true() {
        let mut engine = TemplateEngine::new();
        engine.add_template(
            "conditional",
            "<% if is_member %>Welcome, <%= name %>!<% else %>Please log in.<% endif %>",
        );

        let mut context = Context::new();
        context.insert("is_member", true);
        context.insert("name", "Alice");

        let result = engine
            .render("conditional", &context)
            .expect("Failed to render template");
        assert_eq!(result, "Welcome, Alice!");
    }

    #[test]
    fn test_render_if_else_false() {
        let mut engine = TemplateEngine::new();
        engine.add_template(
            "conditional",
            "<% if is_member %>Welcome, <%= name %>!<% else %>Please log in.<% endif %>",
        );

        let mut context = Context::new();
        context.insert("is_member", false);
        context.insert("name", "Alice");

        let result = engine
            .render("conditional", &context)
            .expect("Failed to render template");
        assert_eq!(result, "Please log in.");
    }

    #[test]
    fn test_render_for_loop() {
        let mut engine = TemplateEngine::new();
        engine.add_template("loop", "<% for item in items %><%= item %> <% endfor %>");

        let mut context = Context::new();
        context.insert("items", vec!["One", "Two", "Three"]);

        let result = engine
            .render("loop", &context)
            .expect("Failed to render template");
        assert_eq!(result.trim(), "One Two Three");
    }

    #[test]
    fn test_render_extend() {
        let mut engine = TemplateEngine::new();
        engine.add_template(
            "base.html",
            "Base content. <% block content %><% endblock %>",
        );
        engine.add_template(
            "child.html",
            "<% extend base.html %><% block content %>Child content<% endblock %>",
        );

        let result = engine
            .render("child.html", &Context::new())
            .expect("Failed to render template");
        assert_eq!(result, "Base content. Child content");
    }

    #[test]
    fn test_render_include() {
        let mut engine = TemplateEngine::new();
        engine.add_template("header.html", "Header content");
        engine.add_template("page.html", "<% include header.html %> Page content");

        let result = engine
            .render("page.html", &Context::new())
            .expect("Failed to render template");
        assert_eq!(result, "Header content Page content");
    }

    #[test]
    fn test_load_templates_from_directory() {
        let temp_dir = "crates/suika_example/templates";

        if std::path::Path::new(temp_dir).exists() {
            fs::remove_dir_all(temp_dir).expect("Failed to remove existing temp directory");
        }

        fs::create_dir_all(format!("{}/ui", temp_dir)).expect("Failed to create temp directory");

        let mut file =
            File::create(format!("{}/index.html", temp_dir)).expect("Failed to create file");
        writeln!(file, "<html><body>Index Template</body></html>")
            .expect("Failed to write to file");

        let mut file =
            File::create(format!("{}/ui/index.html", temp_dir)).expect("Failed to create file");
        writeln!(file, "<html><body>UI Index Template</body></html>")
            .expect("Failed to write to file");

        let mut file =
            File::create(format!("{}/ui/todos.html", temp_dir)).expect("Failed to create file");
        writeln!(file, "<html><body>Todos Template</body></html>")
            .expect("Failed to write to file");

        let mut engine = TemplateEngine::new();
        engine
            .load_templates("crates/suika_example/templates/**/*.html")
            .expect("Failed to load templates");

        let result = engine
            .render("index.html", &Context::new())
            .expect("Failed to render template");
        assert_eq!(result, "<html><body>Index Template</body></html>");

        let result = engine
            .render("ui/index.html", &Context::new())
            .expect("Failed to render template");
        assert_eq!(result, "<html><body>UI Index Template</body></html>");

        let result = engine
            .render("ui/todos.html", &Context::new())
            .expect("Failed to render template");
        assert_eq!(result, "<html><body>Todos Template</body></html>");

        fs::remove_dir_all(temp_dir).expect("Failed to remove temp directory");
    }

    #[test]
    fn test_load_templates_from_nonexistent_directory() {
        let mut engine = TemplateEngine::new();
        let result = engine.load_templates("nonexistent_directory/*.html");
        assert!(
            result.is_err(),
            "Expected error when loading templates from nonexistent directory"
        );
    }

    #[test]
    fn test_load_templates_from_invalid_directory() {
        let temp_file = "temp_file.txt";
        let mut file = File::create(temp_file).expect("Failed to create file");
        writeln!(file, "This is a temp file").expect("Failed to write to file");

        let mut engine = TemplateEngine::new();
        let result = engine.load_templates(temp_file);
        assert!(
            result.is_err(),
            "Expected error when loading templates from an invalid directory"
        );

        fs::remove_file(temp_file).expect("Failed to remove file");
    }

    #[test]
    fn test_render_template_with_object() {
        let mut engine = TemplateEngine::new();
        engine.add_template("greeting", "Hello, <%= user.name %>!");

        let mut user = HashMap::new();
        user.insert("name".to_string(), JsonValue::String("Alice".to_string()));

        let mut context = Context::new();
        context.insert("user", JsonValue::Object(user.into_iter().collect()));

        let result = engine
            .render("greeting", &context)
            .expect("Failed to render template");
        assert_eq!(result, "Hello, Alice!");
    }

    #[test]
    fn test_render_template_with_script() {
        let mut engine = TemplateEngine::new();
        engine.add_template(
            "script_test",
            r#"
            <html>
            <head>
                <title><%= title %></title>
                <script type="module">
                    import init from '/wasm/suika_ui.js';
                    async function loadWasm() {
                        try {
                            await init('/wasm/suika_ui_bg.wasm');
                        } catch (error) {
                            console.error('Failed to load WebAssembly module:', error);
                        }
                    }
                    window.addEventListener('load', loadWasm);
                </script>
            </head>
            <body>
                <h1><%= heading %></h1>
            </body>
            </html>
        "#,
        );

        let mut context = Context::new();
        context.insert("title", "Test Page");
        context.insert("heading", "Welcome!");

        let result = engine
            .render("script_test", &context)
            .expect("Failed to render template");

        // Expected minified output
        let expected = r#"<html><head><title>Test Page</title><script type="module">import init from '/wasm/suika_ui.js';async function loadWasm() {try {await init('/wasm/suika_ui_bg.wasm');} catch (error) {console.error('Failed to load WebAssembly module:', error);}}window.addEventListener('load', loadWasm);</script></head><body><h1>Welcome!</h1></body></html>"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_render_template_with_script_and_loop() {
        let mut engine = TemplateEngine::new();
        engine.add_template(
            "script_loop_test",
            r#"
            <html>
            <head>
                <title><%= title %></title>
                <script type="module">
                    const messages = [<% for message in messages %>"<%= message %>", <% endfor %>];
                    messages.forEach(message => {
                        console.log(message);
                    });

                    <% if is_enabled %>
                    console.log("Feature is enabled");
                    <% else %>
                    console.log("Feature is disabled");
                    <% endif %>
                </script>
            </head>
            <body>
                <h1><%= heading %></h1>
            </body>
            </html>
        "#,
        );

        let mut context = Context::new();
        context.insert("title", "Test Page with Loop");
        context.insert("heading", "Welcome!");
        context.insert("is_enabled", true);
        context.insert("messages", vec!["Message 1", "Message 2", "Message 3"]);

        let result = engine
            .render("script_loop_test", &context)
            .expect("Failed to render template");

        // Expected minified output
        let expected = r#"<html><head><title>Test Page with Loop</title><script type="module">const messages = ["Message 1", "Message 2", "Message 3", ];messages.forEach(message => {console.log(message);});console.log("Feature is enabled");</script></head><body><h1>Welcome!</h1></body></html>"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_render_with_upper_filter() {
        let mut engine = TemplateEngine::new();
        engine.add_template("greeting", "Hello, <%= name|upper %>!");

        let mut context = Context::new();
        context.insert("name", "world");

        let result = engine
            .render("greeting", &context)
            .expect("Failed to render template");
        assert_eq!(result, "Hello, WORLD!");
    }

    #[test]
    fn test_render_with_multiple_filters() {
        let mut engine = TemplateEngine::new();
        engine.add_template("greeting", "Hello, <%= name|lower|upper %>!");

        let mut context = Context::new();
        context.insert("name", "World");

        let result = engine
            .render("greeting", &context)
            .expect("Failed to render template");
        assert_eq!(result, "Hello, WORLD!");
    }

    #[test]
    fn test_render_with_length_filter() {
        let mut engine = TemplateEngine::new();
        engine.add_template("array_length", "Items: <%= items|length %>");

        let mut context = Context::new();
        context.insert("items", vec!["a", "b", "c"]);

        let result = engine
            .render("array_length", &context)
            .expect("Failed to render template");
        assert_eq!(result, "Items: 3");
    }

    #[test]
    fn test_render_macro() {
        let mut engine = TemplateEngine::new();
        engine.add_template(
            "macros.html",
            r#"
            <% macro input(name, type="text") %>
                <input type="<%= type %>" name="<%= name %>" />
            <% endmacro %>
            "#,
        );
        engine.add_template(
            "form.html",
            r#"
            <% include macros.html %>
            <form>
                <% call input("username") %>
                <% call input("password", "password") %>
            </form>
            "#,
        );

        let result = engine
            .render("form.html", &Context::new())
            .expect("Failed to render template");

        let expected = r#"<form><input type="text" name="username" /><input type="password" name="password" /></form>"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_macro_with_context() {
        let mut engine = TemplateEngine::new();
        engine.add_template(
            "macros.html",
            r#"
            <% macro greeting(name) %>
                Hello, <%= name %>!
            <% endmacro %>
            "#,
        );
        engine.add_template(
            "page.html",
            r#"
            <% include macros.html %>
            <div>
                <% call greeting(user_name) %>
            </div>
            "#,
        );

        let mut context = Context::new();
        context.insert("user_name", "Alice");

        let result = engine
            .render("page.html", &context)
            .expect("Failed to render template");

        assert_eq!(result, "<div>Hello, Alice!</div>");
    }
}
