//! # JSON Parser
//!
//! A simple JSON parser library that can parse JSON strings into Rust data structures.
//!
//! This library provides the `JsonValue` enum to represent JSON values and the `JsonParser`
//! struct to parse JSON strings into `JsonValue`.
//!
//! ## Examples
//!
//! Parsing a JSON string:
//!
//! ```
//! use suika_json::{parse_json, JsonValue};
//!
//! let json = r#"{ "key": "value", "array": [1, 2, 3], "number": 123.45, "boolean": true, "null": null }"#;
//! let value = parse_json(json).unwrap();
//!
//! assert_eq!(value, JsonValue::Object(vec![
//!     ("key".to_string(), JsonValue::String("value".to_string())),
//!     ("array".to_string(), JsonValue::Array(vec![
//!         JsonValue::Number(1.0),
//!         JsonValue::Number(2.0),
//!         JsonValue::Number(3.0)
//!     ])),
//!     ("number".to_string(), JsonValue::Number(123.45)),
//!     ("boolean".to_string(), JsonValue::Boolean(true)),
//!     ("null".to_string(), JsonValue::Null)
//! ]));
//! ```

pub mod json_value;
pub mod parser;

pub use json_value::JsonValue;
pub use parser::{parse_json, JsonParser};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_example() {
        let json = r#"{ "key": "value", "array": [1, 2, 3], "number": 123.45, "boolean": true, "null": null }"#;
        let value = parse_json(json).unwrap();

        assert_eq!(
            value,
            JsonValue::Object(vec![
                ("key".to_string(), JsonValue::String("value".to_string())),
                (
                    "array".to_string(),
                    JsonValue::Array(vec![
                        JsonValue::Number(1.0),
                        JsonValue::Number(2.0),
                        JsonValue::Number(3.0)
                    ])
                ),
                ("number".to_string(), JsonValue::Number(123.45)),
                ("boolean".to_string(), JsonValue::Boolean(true)),
                ("null".to_string(), JsonValue::Null)
            ])
        );
    }
}
