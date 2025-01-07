/// Represents a token in a template.
///
/// # Examples
///
/// ```
/// use suika_templates::TemplateToken;
///
/// let text_token = TemplateToken::Text("Hello, world!".to_string());
/// assert_eq!(format!("{:?}", text_token), "Text(\"Hello, world!\")");
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum TemplateToken {
    Text(String),
    Variable(String, Vec<String>),
    If(String),
    Else,
    EndIf,
    For(String, String),
    EndFor,
    Extend(String),
    Include(String),
    Block(String),
    EndBlock,
    MacroDefinition(String, Vec<String>),
    EndMacro,
    MacroCall(String, Vec<String>),
    Raw(String),
    EndRaw,
    Comment(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_token_display() {
        let text_token = TemplateToken::Text("Hello, world!".to_string());
        assert_eq!(format!("{:?}", text_token), "Text(\"Hello, world!\")");

        let variable_token = TemplateToken::Variable("name".to_string(), Vec::new());
        assert_eq!(format!("{:?}", variable_token), "Variable(\"name\", [])");

        let if_token = TemplateToken::If("is_member".to_string());
        assert_eq!(format!("{:?}", if_token), "If(\"is_member\")");
    }
}
