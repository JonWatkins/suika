use std::fmt;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Object(obj) => {
                let entries: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, v))
                    .collect();
                write!(f, "{{{}}}", entries.join(","))
            }
            JsonValue::Array(arr) => {
                let entries: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", entries.join(","))
            }
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::Boolean(b) => write!(f, "{}", b),
            JsonValue::Null => write!(f, "null"),
        }
    }
}

pub struct JsonParser<'a> {
    chars: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> JsonParser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();
        JsonParser {
            chars,
            current_char,
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue, String> {
        self.parse_value()
    }

    fn next_char(&mut self) {
        self.current_char = self.chars.next();
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        match self.current_char {
            Some('{') => self.parse_object(),
            Some('[') => self.parse_array(),
            Some('"') => self.parse_string().map(JsonValue::String),
            Some('t') | Some('f') => self.parse_boolean(),
            Some('n') => self.parse_null(),
            Some('-') | Some('0'..='9') => self.parse_number(),
            Some(c) => Err(format!("Unexpected character: {}", c)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        let mut members = Vec::new();
        self.next_char();

        loop {
            self.skip_whitespace();
            if self.current_char == Some('}') {
                self.next_char();
                break;
            }

            let key = self.parse_string()?;
            self.skip_whitespace();
            if self.current_char != Some(':') {
                return Err("Expected ':' after key in object".to_string());
            }
            self.next_char();
            self.skip_whitespace();
            let value = self.parse_value()?;
            members.push((key, value));
            self.skip_whitespace();

            match self.current_char {
                Some(',') => self.next_char(),
                Some('}') => {
                    self.next_char();
                    break;
                }
                _ => return Err("Expected ',' or '}' in object".to_string()),
            }
        }

        Ok(JsonValue::Object(members))
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        let mut elements = Vec::new();
        self.next_char();

        loop {
            self.skip_whitespace();
            if self.current_char == Some(']') {
                self.next_char();
                break;
            }

            let value = self.parse_value()?;
            elements.push(value);
            self.skip_whitespace();

            match self.current_char {
                Some(',') => self.next_char(),
                Some(']') => {
                    self.next_char();
                    break;
                }
                _ => return Err("Expected ',' or ']' in array".to_string()),
            }
        }

        Ok(JsonValue::Array(elements))
    }

    fn parse_string(&mut self) -> Result<String, String> {
        let mut result = String::new();
        self.next_char();

        while let Some(c) = self.current_char {
            if c == '"' {
                self.next_char();
                return Ok(result);
            } else if c == '\\' {
                self.next_char();
                if let Some(escaped_char) = self.current_char {
                    result.push(match escaped_char {
                        '"' | '\\' | '/' => escaped_char,
                        'b' => '\x08',
                        'f' => '\x0c',
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        _ => return Err(format!("Invalid escape character: {}", escaped_char)),
                    });
                } else {
                    return Err("Unexpected end of input in string escape".to_string());
                }
            } else {
                result.push(c);
            }
            self.next_char();
        }

        Err("Unexpected end of input in string".to_string())
    }

    fn parse_number(&mut self) -> Result<JsonValue, String> {
        let mut number_str = String::new();

        if self.current_char == Some('-') {
            number_str.push('-');
            self.next_char();
        }

        while let Some(c) = self.current_char {
            if c.is_digit(10) || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-' {
                number_str.push(c);
                self.next_char();
            } else {
                break;
            }
        }

        match number_str.parse::<f64>() {
            Ok(num) => Ok(JsonValue::Number(num)),
            Err(_) => Err(format!("Invalid number: {}", number_str)),
        }
    }

    fn parse_boolean(&mut self) -> Result<JsonValue, String> {
        let value = if self.current_char == Some('t') {
            self.expect_sequence("true")?;
            true
        } else {
            self.expect_sequence("false")?;
            false
        };
        Ok(JsonValue::Boolean(value))
    }

    fn parse_null(&mut self) -> Result<JsonValue, String> {
        self.expect_sequence("null")?;
        Ok(JsonValue::Null)
    }

    fn expect_sequence(&mut self, expected: &str) -> Result<(), String> {
        for expected_char in expected.chars() {
            if Some(expected_char) != self.current_char {
                return Err(format!(
                    "Expected '{}', found '{:?}'",
                    expected_char, self.current_char
                ));
            }
            self.next_char();
        }
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }
}

pub fn parse_json(input: &str) -> Result<JsonValue, String> {
    let mut parser = JsonParser::new(input);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let json = r#""hello""#;
        let value = parse_json(json).unwrap();
        assert_eq!(value, JsonValue::String("hello".to_string()));
    }

    #[test]
    fn test_parse_number() {
        let json = "123.45";
        let value = parse_json(json).unwrap();
        assert_eq!(value, JsonValue::Number(123.45));
    }

    #[test]
    fn test_parse_boolean() {
        let json = "true";
        let value = parse_json(json).unwrap();
        assert_eq!(value, JsonValue::Boolean(true));

        let json = "false";
        let value = parse_json(json).unwrap();
        assert_eq!(value, JsonValue::Boolean(false));
    }

    #[test]
    fn test_parse_null() {
        let json = "null";
        let value = parse_json(json).unwrap();
        assert_eq!(value, JsonValue::Null);
    }

    #[test]
    fn test_parse_array() {
        let json = "[1, 2, 3]";
        let value = parse_json(json).unwrap();
        assert_eq!(
            value,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0)
            ])
        );
    }

    #[test]
    fn test_parse_object() {
        let json = r#"{"key": "value"}"#;
        let value = parse_json(json).unwrap();
        assert_eq!(
            value,
            JsonValue::Object(vec![(
                "key".to_string(),
                JsonValue::String("value".to_string())
            )])
        );
    }
}
