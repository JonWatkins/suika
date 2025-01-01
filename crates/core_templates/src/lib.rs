use std::collections::HashMap;
use std::fmt;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum TemplateToken {
    Text(String),
    Variable(String),
    If(String),
    Else,
    EndIf,
    For(String, String),
    EndFor,
    Extend(String),
    Include(String),
    Block(String),
    EndBlock,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TemplateValue {
    String(String),
    Boolean(bool),
    Array(Vec<TemplateValue>),
}

impl fmt::Display for TemplateValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateValue::String(s) => write!(f, "{}", s),
            TemplateValue::Boolean(b) => write!(f, "{}", b),
            TemplateValue::Array(arr) => {
                let entries: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", entries.join(", "))
            }
        }
    }
}

pub struct TemplateParser<'a> {
    chars: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> TemplateParser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();
        TemplateParser {
            chars,
            current_char,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<TemplateToken>, String> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        Ok(tokens)
    }

    fn next_char(&mut self) {
        self.current_char = self.chars.next();
    }

    fn next_token(&mut self) -> Result<Option<TemplateToken>, String> {
        match self.current_char {
            Some('{') => {
                self.next_char();
                if self.current_char == Some('{') {
                    self.next_char();
                    self.parse_variable()
                } else if self.current_char == Some('%') {
                    self.next_char();
                    self.parse_directive()
                } else {
                    Err("Unexpected character after '{'".to_string())
                }
            }
            Some(_) => self.parse_text(),
            None => Ok(None),
        }
    }

    fn parse_text(&mut self) -> Result<Option<TemplateToken>, String> {
        let mut text = String::new();
        while let Some(c) = self.current_char {
            if c == '{' {
                break;
            }
            text.push(c);
            self.next_char();
        }
        Ok(Some(TemplateToken::Text(text)))
    }

    fn parse_variable(&mut self) -> Result<Option<TemplateToken>, String> {
        let mut var_name = String::new();
        while let Some(c) = self.current_char {
            if c == '}' && self.chars.as_str().starts_with('}') {
                self.next_char();
                self.next_char();
                return Ok(Some(TemplateToken::Variable(var_name.trim().to_string())));
            }
            var_name.push(c);
            self.next_char();
        }
        Err("Unexpected end of input in variable".to_string())
    }

    fn parse_directive(&mut self) -> Result<Option<TemplateToken>, String> {
        let mut directive = String::new();
        while let Some(c) = self.current_char {
            if c == '%' && self.chars.as_str().starts_with('}') {
                self.next_char();
                self.next_char();
                directive = directive.trim().to_string();
                if directive.starts_with("if ") {
                    return Ok(Some(TemplateToken::If(directive[3..].to_string())));
                } else if directive == "else" {
                    return Ok(Some(TemplateToken::Else));
                } else if directive == "endif" {
                    return Ok(Some(TemplateToken::EndIf));
                } else if directive.starts_with("for ") {
                    let parts: Vec<&str> = directive[4..].split_whitespace().collect();
                    if parts.len() == 3 && parts[1] == "in" {
                        return Ok(Some(TemplateToken::For(
                            parts[0].to_string(),
                            parts[2].to_string(),
                        )));
                    }
                    return Err(format!("Invalid for directive: {}", directive));
                } else if directive == "endfor" {
                    return Ok(Some(TemplateToken::EndFor));
                } else if directive.starts_with("extend ") {
                    return Ok(Some(TemplateToken::Extend(
                        directive[7..].trim().to_string(),
                    )));
                } else if directive.starts_with("include ") {
                    return Ok(Some(TemplateToken::Include(
                        directive[8..].trim().to_string(),
                    )));
                } else if directive.starts_with("block ") {
                    return Ok(Some(TemplateToken::Block(
                        directive[6..].trim().to_string(),
                    )));
                } else if directive == "endblock" {
                    return Ok(Some(TemplateToken::EndBlock));
                } else {
                    return Err(format!("Unknown directive: {}", directive));
                }
            }
            directive.push(c);
            self.next_char();
        }
        Err("Unexpected end of input in directive".to_string())
    }
}

#[derive(Clone)]
pub struct TemplateEngine {
    templates: HashMap<String, String>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    pub fn add_template(&mut self, name: &str, content: &str) {
        self.templates.insert(name.to_string(), content.to_string());
    }

    pub fn load_templates_from_directory(&mut self, dir: &str) -> Result<(), String> {
        let path = std::path::Path::new(dir);
        if !path.is_dir() {
            return Err(format!("{} is not a directory", dir));
        }

        for entry in std::fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "html" {
                        if let Some(template_name) = path.file_name().and_then(|n| n.to_str()) {
                            let content =
                                std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
                            self.add_template(template_name, content.trim());
                        }
                    }
                }
            }
        }
        Ok(())
    }

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

        self.process_tokens(&tokens, context)
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
                TemplateToken::Text(text) => output.push_str(text),
                TemplateToken::Variable(name) => {
                    if let Some(value) = context.get(name) {
                        output.push_str(&value.to_string());
                    }
                }
                TemplateToken::If(condition) => {
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
                }
                TemplateToken::For(var, array) => {
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
                            loop_context.insert(var.clone(), value.clone());
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
                }
                TemplateToken::Include(template_name) => {
                    let include_tokens = self.get_template_tokens(template_name)?;
                    output.push_str(&self.process_tokens(&include_tokens, context)?);
                }
                TemplateToken::EndIf
                | TemplateToken::EndFor
                | TemplateToken::Extend(_)
                | TemplateToken::Block(_)
                | TemplateToken::EndBlock => {}
                TemplateToken::Else => {}
            }
            i += 1;
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_parse_text() {
        let mut parser = TemplateParser::new("Hello, world!");
        let tokens = parser.parse().unwrap();
        assert_eq!(
            tokens,
            vec![TemplateToken::Text("Hello, world!".to_string())]
        );
    }

    #[test]
    fn test_parse_variable() {
        let mut parser = TemplateParser::new("Hello, {{ name }}!");
        let tokens = parser.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                TemplateToken::Text("Hello, ".to_string()),
                TemplateToken::Variable("name".to_string()),
                TemplateToken::Text("!".to_string())
            ]
        );
    }

    #[test]
    fn test_parse_if() {
        let mut parser = TemplateParser::new("{% if is_member %}Welcome!{% endif %}");
        let tokens = parser.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                TemplateToken::If("is_member".to_string()),
                TemplateToken::Text("Welcome!".to_string()),
                TemplateToken::EndIf
            ]
        );
    }

    #[test]
    fn test_parse_if_else() {
        let mut parser =
            TemplateParser::new("{% if is_member %}Welcome!{% else %}Please log in.{% endif %}");
        let tokens = parser.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                TemplateToken::If("is_member".to_string()),
                TemplateToken::Text("Welcome!".to_string()),
                TemplateToken::Else,
                TemplateToken::Text("Please log in.".to_string()),
                TemplateToken::EndIf
            ]
        );
    }

    #[test]
    fn test_parse_for() {
        let mut parser = TemplateParser::new("{% for item in items %} {{ item }} {% endfor %}");
        let tokens = parser.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                TemplateToken::For("item".to_string(), "items".to_string()),
                TemplateToken::Text(" ".to_string()),
                TemplateToken::Variable("item".to_string()),
                TemplateToken::Text(" ".to_string()),
                TemplateToken::EndFor
            ]
        );
    }

    #[test]
    fn test_parse_extend() {
        let mut parser = TemplateParser::new("{% extend base.html %}");
        let tokens = parser.parse().unwrap();
        assert_eq!(tokens, vec![TemplateToken::Extend("base.html".to_string())]);
    }

    #[test]
    fn test_parse_include() {
        let mut parser = TemplateParser::new("{% include header.html %}");
        let tokens = parser.parse().unwrap();
        assert_eq!(
            tokens,
            vec![TemplateToken::Include("header.html".to_string())]
        );
    }

    #[test]
    fn test_parse_block() {
        let mut parser = TemplateParser::new("{% block content %}Block content{% endblock %}");
        let tokens = parser.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                TemplateToken::Block("content".to_string()),
                TemplateToken::Text("Block content".to_string()),
                TemplateToken::EndBlock
            ]
        );
    }

    #[test]
    fn test_render_variable() {
        let mut engine = TemplateEngine::new();
        engine.add_template("hello", "Hello, {{ name }}!");

        let mut context = HashMap::new();
        context.insert(
            "name".to_string(),
            TemplateValue::String("World".to_string()),
        );

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
            "{% if is_member %}Welcome, {{ name }}!{% endif %}",
        );

        let mut context = HashMap::new();
        context.insert("is_member".to_string(), TemplateValue::Boolean(true));
        context.insert(
            "name".to_string(),
            TemplateValue::String("Alice".to_string()),
        );

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
            "{% if is_member %}Welcome, {{ name }}!{% endif %}",
        );

        let mut context = HashMap::new();
        context.insert("is_member".to_string(), TemplateValue::Boolean(false));
        context.insert(
            "name".to_string(),
            TemplateValue::String("Alice".to_string()),
        );

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
            "{% if is_member %}Welcome, {{ name }}!{% else %}Please log in.{% endif %}",
        );

        let mut context = HashMap::new();
        context.insert("is_member".to_string(), TemplateValue::Boolean(true));
        context.insert(
            "name".to_string(),
            TemplateValue::String("Alice".to_string()),
        );

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
            "{% if is_member %}Welcome, {{ name }}!{% else %}Please log in.{% endif %}",
        );

        let mut context = HashMap::new();
        context.insert("is_member".to_string(), TemplateValue::Boolean(false));
        context.insert(
            "name".to_string(),
            TemplateValue::String("Alice".to_string()),
        );

        let result = engine
            .render("conditional", &context)
            .expect("Failed to render template");
        assert_eq!(result, "Please log in.");
    }

    #[test]
    fn test_render_for_loop() {
        let mut engine = TemplateEngine::new();
        engine.add_template("loop", "{% for item in items %} {{ item }} {% endfor %}");

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
        assert_eq!(result, " One  Two  Three ");
    }

    #[test]
    fn test_render_extend() {
        let mut engine = TemplateEngine::new();
        engine.add_template(
            "base.html",
            "Base content. {% block content %}{% endblock %}",
        );
        engine.add_template(
            "child.html",
            "{% extend base.html %}{% block content %}Child content{% endblock %}",
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
        engine.add_template("page.html", "{% include header.html %} Page content");

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
}
