use std::collections::HashMap;
use std::fmt;

/// Represents a JSON value (object, array, string, number, boolean, or null).
///
/// # Examples
///
/// ```
/// use suika_json::JsonValue;
///
/// let json_string = JsonValue::String("hello".to_string());
/// assert_eq!(json_string.to_string(), "\"hello\"");
///
/// let json_number = JsonValue::Number(123.45);
/// assert_eq!(json_number.to_string(), "123.45");
///
/// let json_boolean = JsonValue::Boolean(true);
/// assert_eq!(json_boolean.to_string(), "true");
///
/// let json_null = JsonValue::Null;
/// assert_eq!(json_null.to_string(), "null");
///
/// let json_array = JsonValue::Array(vec![
///     JsonValue::Number(1.0),
///     JsonValue::Number(2.0),
///     JsonValue::Number(3.0)
/// ]);
/// assert_eq!(json_array.to_string(), "[1,2,3]");
///
/// let json_object = JsonValue::Object(vec![
///     ("key".to_string(), JsonValue::String("value".to_string()))
/// ]);
/// assert_eq!(json_object.to_string(), "{\"key\":\"value\"}");
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl JsonValue {
    /// Serializes the `JsonValue` to a JSON string.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_json::JsonValue;
    ///
    /// let json_string = JsonValue::String("hello".to_string());
    /// assert_eq!(json_string.to_string(), "\"hello\"");
    ///
    /// let json_number = JsonValue::Number(123.45);
    /// assert_eq!(json_number.to_string(), "123.45");
    ///
    /// let json_boolean = JsonValue::Boolean(true);
    /// assert_eq!(json_boolean.to_string(), "true");
    ///
    /// let json_null = JsonValue::Null;
    /// assert_eq!(json_null.to_string(), "null");
    ///
    /// let json_array = JsonValue::Array(vec![
    ///     JsonValue::Number(1.0),
    ///     JsonValue::Number(2.0),
    ///     JsonValue::Number(3.0)
    /// ]);
    /// assert_eq!(json_array.to_string(), "[1,2,3]");
    ///
    /// let json_object = JsonValue::Object(vec![
    ///     ("key".to_string(), JsonValue::String("value".to_string()))
    /// ]);
    /// assert_eq!(json_object.to_string(), "{\"key\":\"value\"}");
    /// ```
    pub fn to_string(&self) -> String {
        match self {
            JsonValue::Object(obj) => {
                let entries: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", entries.join(","))
            }
            JsonValue::Array(arr) => {
                let entries: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", entries.join(","))
            }
            JsonValue::String(s) => format!("\"{}\"", s),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::Boolean(b) => b.to_string(),
            JsonValue::Null => "null".to_string(),
        }
    }
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

impl From<String> for JsonValue {
    fn from(value: String) -> Self {
        JsonValue::String(value)
    }
}

impl From<&str> for JsonValue {
    fn from(value: &str) -> Self {
        JsonValue::String(value.to_string())
    }
}

impl From<f64> for JsonValue {
    fn from(value: f64) -> Self {
        JsonValue::Number(value)
    }
}

impl From<i64> for JsonValue {
    fn from(value: i64) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<bool> for JsonValue {
    fn from(value: bool) -> Self {
        JsonValue::Boolean(value)
    }
}

impl<T: Into<JsonValue>> From<Vec<T>> for JsonValue {
    fn from(value: Vec<T>) -> Self {
        JsonValue::Array(value.into_iter().map(|item| item.into()).collect())
    }
}

impl From<HashMap<String, JsonValue>> for JsonValue {
    fn from(value: HashMap<String, JsonValue>) -> Self {
        JsonValue::Object(value.into_iter().collect())
    }
}

impl From<usize> for JsonValue {
    fn from(value: usize) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<i32> for JsonValue {
    fn from(value: i32) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<i16> for JsonValue {
    fn from(value: i16) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<i8> for JsonValue {
    fn from(value: i8) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<u64> for JsonValue {
    fn from(value: u64) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<u32> for JsonValue {
    fn from(value: u32) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<u16> for JsonValue {
    fn from(value: u16) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<u8> for JsonValue {
    fn from(value: u8) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<f32> for JsonValue {
    fn from(value: f32) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl<T: Into<JsonValue>, const N: usize> From<[T; N]> for JsonValue {
    fn from(arr: [T; N]) -> Self {
        JsonValue::Array(arr.into_iter().map(|x| x.into()).collect())
    }
}

impl From<()> for JsonValue {
    fn from(_: ()) -> Self {
        JsonValue::Null
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_value_display() {
        let json_string = JsonValue::String("hello".to_string());
        assert_eq!(json_string.to_string(), "\"hello\"");

        let json_number = JsonValue::Number(123.45);
        assert_eq!(json_number.to_string(), "123.45");

        let json_boolean = JsonValue::Boolean(true);
        assert_eq!(json_boolean.to_string(), "true");

        let json_null = JsonValue::Null;
        assert_eq!(json_null.to_string(), "null");

        let json_array = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]);
        assert_eq!(json_array.to_string(), "[1,2,3]");

        let json_object = JsonValue::Object(vec![(
            "key".to_string(),
            JsonValue::String("value".to_string()),
        )]);
        assert_eq!(json_object.to_string(), "{\"key\":\"value\"}");

        // Test nested objects
        let nested_object = JsonValue::Object(vec![
            (
                "user".to_string(),
                JsonValue::Object(vec![
                    (
                        "name".to_string(),
                        JsonValue::String("John Doe".to_string()),
                    ),
                    ("age".to_string(), JsonValue::Number(30.0)),
                    ("is_student".to_string(), JsonValue::Boolean(false)),
                ]),
            ),
            (
                "courses".to_string(),
                JsonValue::Array(vec![
                    JsonValue::String("Math".to_string()),
                    JsonValue::String("Science".to_string()),
                ]),
            ),
        ]);
        assert_eq!(
            nested_object.to_string(),
            r#"{"user":{"name":"John Doe","age":30,"is_student":false},"courses":["Math","Science"]}"#
        );

        // Test empty object
        let empty_object = JsonValue::Object(vec![]);
        assert_eq!(empty_object.to_string(), "{}");

        // Test empty array
        let empty_array = JsonValue::Array(vec![]);
        assert_eq!(empty_array.to_string(), "[]");
    }

    #[test]
    fn test_from_string() {
        let json = JsonValue::from("hello");
        assert_eq!(json, JsonValue::String("hello".to_string()));
    }

    #[test]
    fn test_from_i64() {
        let json = JsonValue::from(42i64);
        assert_eq!(json, JsonValue::Number(42.0));
    }

    #[test]
    fn test_from_f64() {
        let json = JsonValue::from(42.0f64);
        assert_eq!(json, JsonValue::Number(42.0));
    }

    #[test]
    fn test_from_bool() {
        let json = JsonValue::from(true);
        assert_eq!(json, JsonValue::Boolean(true));
    }

    #[test]
    fn test_from_vec() {
        let json = JsonValue::from(vec!["One", "Two", "Three"]);
        assert_eq!(
            json,
            JsonValue::Array(vec![
                JsonValue::String("One".to_string()),
                JsonValue::String("Two".to_string()),
                JsonValue::String("Three".to_string())
            ])
        );
    }

    #[test]
    fn test_from_hashmap() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), JsonValue::String("value1".to_string()));
        map.insert("key2".to_string(), JsonValue::Number(42.0));

        let json = JsonValue::from(map);

        if let JsonValue::Object(obj) = json {
            let mut obj_map: HashMap<String, JsonValue> = obj.into_iter().collect();
            assert_eq!(obj_map.remove("key1"), Some(JsonValue::String("value1".to_string())));
            assert_eq!(obj_map.remove("key2"), Some(JsonValue::Number(42.0)));
            assert!(obj_map.is_empty());
        } else {
            panic!("Expected JsonValue::Object");
        }
    }

    #[test]
    fn test_from_various_number_types() {
        assert_eq!(JsonValue::from(42i32), JsonValue::Number(42.0));
        assert_eq!(JsonValue::from(42i16), JsonValue::Number(42.0));
        assert_eq!(JsonValue::from(42i8), JsonValue::Number(42.0));
        assert_eq!(JsonValue::from(42u64), JsonValue::Number(42.0));
        assert_eq!(JsonValue::from(42u32), JsonValue::Number(42.0));
        assert_eq!(JsonValue::from(42u16), JsonValue::Number(42.0));
        assert_eq!(JsonValue::from(42u8), JsonValue::Number(42.0));
        assert_eq!(JsonValue::from(42.0f32), JsonValue::Number(42.0));
    }
}