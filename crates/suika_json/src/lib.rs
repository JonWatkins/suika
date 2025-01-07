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
pub const NULL: () = ();

#[macro_export]
macro_rules! json {
    ({ $($key:expr => $value:expr),* $(,)? }) => {{
        let mut map = Vec::new();
        $(
            map.push(($key.to_string(), json!($value)));
        )*
        $crate::JsonValue::Object(map)
    }};

    ([ $($value:expr),* $(,)? ]) => {{
        let mut vec = Vec::new();
        $(
            vec.push(json!($value));
        )*
        $crate::JsonValue::Array(vec)
    }};

    (NULL) => { $crate::JsonValue::Null };

    ($other:expr) => {
        $crate::JsonValue::from($other)
    };
}

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

    #[test]
    fn test_json_macro() {
        // Test object creation
        let obj = json!({
            "name" => "Alice",
            "age" => 30,
            "is_student" => true,
            "grades" => [85, 92, 88]
        });

        match obj {
            JsonValue::Object(map) => {
                assert_eq!(map.len(), 4);
                assert_eq!(map.iter().find(|(k, _)| k == "name").unwrap().1, JsonValue::String("Alice".to_string()));
                assert_eq!(map.iter().find(|(k, _)| k == "age").unwrap().1, JsonValue::Number(30.0));
                assert_eq!(map.iter().find(|(k, _)| k == "is_student").unwrap().1, JsonValue::Boolean(true));
            },
            _ => panic!("Expected object"),
        }

        // Test array creation
        let arr = json!([1, 2, "three", true, NULL]);
        match arr {
            JsonValue::Array(vec) => {
                assert_eq!(vec.len(), 5);
                assert_eq!(vec[0], JsonValue::Number(1.0));
                assert_eq!(vec[1], JsonValue::Number(2.0));
                assert_eq!(vec[2], JsonValue::String("three".to_string()));
                assert_eq!(vec[3], JsonValue::Boolean(true));
                assert_eq!(vec[4], JsonValue::Null);
            },
            _ => panic!("Expected array"),
        }

        // Test single value
        assert_eq!(json!(42), JsonValue::Number(42.0));
        assert_eq!(json!("hello"), JsonValue::String("hello".to_string()));
        assert_eq!(json!(true), JsonValue::Boolean(true));
        assert_eq!(json!(NULL), JsonValue::Null);
    }
}
