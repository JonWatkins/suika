use crate::template_token::TemplateToken;
use std::str::Chars;

/// A parser for template strings.
///
/// # Examples
///
/// ```
/// use suika_templates::TemplateParser;
/// use suika_templates::TemplateToken;
///
/// let mut parser = TemplateParser::new("Hello, <%= name %>!");
/// let tokens = parser.parse().unwrap();
///
/// assert_eq!(tokens, vec![
///     TemplateToken::Text("Hello, ".to_string()),
///     TemplateToken::Variable("name".to_string()),
///     TemplateToken::Text("!".to_string())
/// ]);
/// ```
pub struct TemplateParser<'a> {
    chars: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> TemplateParser<'a> {
    /// Creates a new `TemplateParser` from an input string.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the template input.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_templates::TemplateParser;
    ///
    /// let parser = TemplateParser::new("Hello, <%= name %>!");
    /// ```
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();
        TemplateParser {
            chars,
            current_char,
        }
    }

    /// Parses the input template string and returns a vector of `TemplateToken`.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `TemplateToken` or an error message.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_templates::{TemplateParser, TemplateToken};
    ///
    /// let mut parser = TemplateParser::new("Hello, <%= name %>!");
    /// let tokens = parser.parse().unwrap();
    ///
    /// assert_eq!(tokens, vec![
    ///     TemplateToken::Text("Hello, ".to_string()),
    ///     TemplateToken::Variable("name".to_string()),
    ///     TemplateToken::Text("!".to_string())
    /// ]);
    /// ```
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
            Some('<') => {
                self.next_char();
                if self.current_char == Some('%') {
                    self.next_char();
                    self.parse_template_directive()
                } else {
                    self.parse_text('<')
                }
            }
            Some(_) => self.parse_text(' '),
            None => Ok(None),
        }
    }

    fn parse_text(&mut self, initial_char: char) -> Result<Option<TemplateToken>, String> {
        let mut text = String::new();
        if !initial_char.is_whitespace() {
            text.push(initial_char);
        }
        while let Some(c) = self.current_char {
            if c == '<' && self.chars.as_str().starts_with("%") {
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
            if c == '%' && self.chars.as_str().starts_with(">") {
                self.next_char();
                self.next_char();
                return Ok(Some(TemplateToken::Variable(var_name.trim().to_string())));
            }
            var_name.push(c);
            self.next_char();
        }
        Err("Unexpected end of input in variable".to_string())
    }

    fn parse_template_directive(&mut self) -> Result<Option<TemplateToken>, String> {
        match self.current_char {
            Some('=') => {
                self.next_char();
                self.parse_variable()
            }
            Some(_) => self.parse_directive(),
            None => Err("Unexpected end of input in template directive".to_string()),
        }
    }

    fn parse_directive(&mut self) -> Result<Option<TemplateToken>, String> {
        let mut directive = String::new();
        while let Some(c) = self.current_char {
            if c == '%' && self.chars.as_str().starts_with(">") {
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

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut parser = TemplateParser::new("Hello, <%= name %>!");
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
        let mut parser = TemplateParser::new("<% if is_member %>Welcome!<% endif %>");
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
            TemplateParser::new("<% if is_member %>Welcome!<% else %>Please log in.<% endif %>");
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
        let mut parser = TemplateParser::new("<% for item in items %> <%= item %> <% endfor %>");
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
        let mut parser = TemplateParser::new("<% extend base.html %>");
        let tokens = parser.parse().unwrap();
        assert_eq!(tokens, vec![TemplateToken::Extend("base.html".to_string())]);
    }

    #[test]
    fn test_parse_include() {
        let mut parser = TemplateParser::new("<% include header.html %>");
        let tokens = parser.parse().unwrap();
        assert_eq!(
            tokens,
            vec![TemplateToken::Include("header.html".to_string())]
        );
    }

    #[test]
    fn test_parse_block() {
        let mut parser = TemplateParser::new("<% block content %>Block content<% endblock %>");
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
}
