//! # TemplateEngine
//!
//! The `TemplateEngine` struct provides methods to manage and render templates with
//! various directives and context values.

use super::{TemplateParser, TemplateToken, TemplateValue};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use suika_utils::minify_html;

#[derive(Debug, Clone)]
pub struct TemplateEngine {
    templates: HashMap<String, String>,
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
        self.templates.insert(name.to_string(), content.to_string());
    }

    /// Loads templates from the specified directory.
    ///
    /// # Arguments
    ///
    /// * `dir` - The path to the directory containing the templates.
    ///
    /// # Errors
    ///
    /// Returns an error if the specified path is not a directory or if there is an issue reading the files.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_templates::template_engine::TemplateEngine;
    /// use std::fs::{self, File};
    /// use std::io::Write;
    ///
    /// // Create a temporary directory for testing
    /// let temp_dir = "temp_templates";
    /// fs::create_dir(temp_dir).expect("Failed to create temp directory");
    ///
    /// // Create temporary template files
    /// let mut file = File::create(format!("{}/template1.html", temp_dir)).expect("Failed to create file");
    /// writeln!(file, "<html><body>Template 1</body></html>").expect("Failed to write to file");
    ///
    /// let mut file = File::create(format!("{}/template2.html", temp_dir)).expect("Failed to create file");
    /// writeln!(file, "<html><body>Template 2</body></html>").expect("Failed to write to file");
    ///
    /// // Load templates from the temporary directory
    /// let mut engine = TemplateEngine::new();
    /// engine.load_templates_from_directory(temp_dir).expect("Failed to load templates");
    ///
    /// // Clean up temporary files
    /// fs::remove_file(format!("{}/template1.html", temp_dir)).expect("Failed to remove file");
    /// fs::remove_file(format!("{}/template2.html", temp_dir)).expect("Failed to remove file");
    /// fs::remove_dir(temp_dir).expect("Failed to remove directory");
    /// ```
    pub fn load_templates_from_directory(&mut self, dir: &str) -> Result<(), String> {
        let path = Path::new(dir);
        if !path.is_dir() {
            return Err(format!("{} is not a directory", dir));
        }

        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "html" {
                        if let Some(template_name) = path.file_name().and_then(|n| n.to_str()) {
                            let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
                            self.add_template(template_name, content.trim());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Renders a template with the given name and context.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the template to render.
    /// * `context` - A `HashMap` containing the context values to use in the template.
    ///
    /// # Errors
    ///
    /// Returns an error if the template is not found or if there is an issue processing the template.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_templates::template_engine::TemplateEngine;
    /// use suika_templates::template_value::TemplateValue;
    /// use std::collections::HashMap;
    ///
    /// let mut engine = TemplateEngine::new();
    /// engine.add_template("hello", "Hello, <%= name %>!");
    ///
    /// let mut context = HashMap::new();
    /// context.insert("name".to_string(), TemplateValue::String("World".to_string()));
    ///
    /// let result = engine.render("hello", &context).expect("Failed to render template");
    /// assert_eq!(result, "Hello, World!");
    /// ```
    pub fn render(
        &self,
        name: &str,
        context: &HashMap<String, TemplateValue>,
    ) -> Result<String, String> {
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
        context: &HashMap<String, TemplateValue>,
    ) -> Result<String, String> {
        let mut output = String::new();
        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                TemplateToken::Text(text) => self.process_text(text, &mut output),
                TemplateToken::Variable(name) => {
                    self.process_variable(name, context, &mut output)?
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
        context: &HashMap<String, TemplateValue>,
        output: &mut String,
    ) -> Result<(), String> {
        if let Some(value) = self.resolve_variable(name, context) {
            output.push_str(&value);
        }
        Ok(())
    }

    fn process_if(
        &self,
        condition: &str,
        tokens: &[TemplateToken],
        context: &HashMap<String, TemplateValue>,
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
        if let Some(TemplateValue::Boolean(true)) = context.get(condition) {
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
        context: &HashMap<String, TemplateValue>,
        output: &mut String,
        mut i: usize,
    ) -> Result<usize, String> {
        if let Some(TemplateValue::Array(values)) = context.get(array) {
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
                loop_context.insert(var.to_string(), value.clone());
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
        context: &HashMap<String, TemplateValue>,
        output: &mut String,
    ) -> Result<(), String> {
        let include_tokens = self.get_template_tokens(template_name)?;
        output.push_str(&self.process_tokens(&include_tokens, context)?);
        Ok(())
    }

    fn resolve_variable(
        &self,
        name: &str,
        context: &HashMap<String, TemplateValue>,
    ) -> Option<String> {
        let parts: Vec<&str> = name.split('.').collect();
        let mut current_value = context.get(parts[0])?;

        for part in &parts[1..] {
            match current_value {
                TemplateValue::Object(map) => {
                    current_value = map.get(*part)?;
                }
                _ => return None,
            }
        }

        match current_value {
            TemplateValue::String(s) => Some(s.clone()),
            TemplateValue::Boolean(b) => Some(b.to_string()),
            TemplateValue::Array(arr) => Some(format!(
                "[{}]",
                arr.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )),
            TemplateValue::Object(obj) => Some(format!(
                "{{{}}}",
                obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ")
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_render_variable() {
        let mut engine = TemplateEngine::new();
        engine.add_template("hello", "Hello, <%= name %>!");

        let mut context = HashMap::new();
        context.insert("name".to_string(), TemplateValue::String("World".to_string()));

        let result = engine
            .render("hello", &context)
            .expect("Failed to render template");
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_render_conditional_true() {
        let mut engine = TemplateEngine::new();
        engine.add_template(
            "conditional",
            "<% if is_member %>Welcome, <%= name %>!<% endif %>",
        );

        let mut context = HashMap::new();
        context.insert("is_member".to_string(), TemplateValue::Boolean(true));
        context.insert("name".to_string(), TemplateValue::String("Alice".to_string()));

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

        let mut context = HashMap::new();
        context.insert("is_member".to_string(), TemplateValue::Boolean(false));
        context.insert("name".to_string(), TemplateValue::String("Alice".to_string()));

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

        let mut context = HashMap::new();
        context.insert("is_member".to_string(), TemplateValue::Boolean(true));
        context.insert("name".to_string(), TemplateValue::String("Alice".to_string()));

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

        let mut context = HashMap::new();
        context.insert("is_member".to_string(), TemplateValue::Boolean(false));
        context.insert("name".to_string(), TemplateValue::String("Alice".to_string()));

        let result = engine
            .render("conditional", &context)
            .expect("Failed to render template");
        assert_eq!(result, "Please log in.");
    }

    #[test]
    fn test_render_for_loop() {
        let mut engine = TemplateEngine::new();
        engine.add_template("loop", "<% for item in items %><%= item %> <% endfor %>");

        let mut context = HashMap::new();
        context.insert(
            "items".to_string(),
            TemplateValue::Array(vec![
                TemplateValue::String("One".to_string()),
                TemplateValue::String("Two".to_string()),
                TemplateValue::String("Three".to_string()),
            ]),
        );

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
            .render("child.html", &HashMap::new())
            .expect("Failed to render template");
        assert_eq!(result, "Base content. Child content");
    }

    #[test]
    fn test_render_include() {
        let mut engine = TemplateEngine::new();
        engine.add_template("header.html", "Header content");
        engine.add_template("page.html", "<% include header.html %> Page content");

        let result = engine
            .render("page.html", &HashMap::new())
            .expect("Failed to render template");
        assert_eq!(result, "Header content Page content");
    }

    #[test]
    fn test_load_templates_from_directory() {
        let temp_dir = "temp_templates";

        if std::path::Path::new(temp_dir).exists() {
            fs::remove_dir_all(temp_dir).expect("Failed to remove existing temp directory");
        }

        fs::create_dir(temp_dir).expect("Failed to create temp directory");

        let mut file =
            File::create(format!("{}/template1.html", temp_dir)).expect("Failed to create file");
        writeln!(file, "<html><body>Template 1</body></html>").expect("Failed to write to file");

        let mut file =
            File::create(format!("{}/template2.html", temp_dir)).expect("Failed to create file");
        writeln!(file, "<html><body>Template 2</body></html>").expect("Failed to write to file");

        let mut engine = TemplateEngine::new();
        engine
            .load_templates_from_directory(temp_dir)
            .expect("Failed to load templates");

        let result = engine
            .render("template1.html", &HashMap::new())
            .expect("Failed to render template");
        assert_eq!(result, "<html><body>Template 1</body></html>");

        let result = engine
            .render("template2.html", &HashMap::new())
            .expect("Failed to render template");
        assert_eq!(result, "<html><body>Template 2</body></html>");

        fs::remove_file(format!("{}/template1.html", temp_dir)).expect("Failed to remove file");
        fs::remove_file(format!("{}/template2.html", temp_dir)).expect("Failed to remove file");
        fs::remove_dir(temp_dir).expect("Failed to remove directory");
    }

    #[test]
    fn test_load_templates_from_nonexistent_directory() {
        let mut engine = TemplateEngine::new();
        let result = engine.load_templates_from_directory("nonexistent_directory");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_templates_from_invalid_directory() {
        let temp_file = "temp_file.txt";
        let mut file = File::create(temp_file).expect("Failed to create file");
        writeln!(file, "This is a temp file").expect("Failed to write to file");

        let mut engine = TemplateEngine::new();
        let result = engine.load_templates_from_directory(temp_file);
        assert!(result.is_err());

        fs::remove_file(temp_file).expect("Failed to remove file");
    }

    #[test]
    fn test_render_template_with_object() {
        let mut engine = TemplateEngine::new();
        engine.add_template("greeting", "Hello, <%= user.name %>!");

        let mut user = HashMap::new();
        user.insert("name".to_string(), TemplateValue::String("Alice".to_string()));

        let mut context = HashMap::new();
        context.insert("user".to_string(), TemplateValue::Object(user));

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

        let mut context = HashMap::new();
        context.insert("title".to_string(), TemplateValue::String("Test Page".to_string()));
        context.insert("heading".to_string(), TemplateValue::String("Welcome!".to_string()));

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

        let mut context = HashMap::new();
        context.insert("title".to_string(), TemplateValue::String("Test Page with Loop".to_string()));
        context.insert("heading".to_string(), TemplateValue::String("Welcome!".to_string()));
        context.insert("is_enabled".to_string(), TemplateValue::Boolean(true));
        context.insert(
            "messages".to_string(),
            TemplateValue::Array(vec![
                TemplateValue::String("Message 1".to_string()),
                TemplateValue::String("Message 2".to_string()),
                TemplateValue::String("Message 3".to_string()),
            ]),
        );

        let result = engine
            .render("script_loop_test", &context)
            .expect("Failed to render template");

        // Expected minified output
        let expected = r#"<html><head><title>Test Page with Loop</title><script type="module">const messages = ["Message 1", "Message 2", "Message 3", ];messages.forEach(message => {console.log(message);});console.log("Feature is enabled");</script></head><body><h1>Welcome!</h1></body></html>"#;

        assert_eq!(result, expected);
    }
}
