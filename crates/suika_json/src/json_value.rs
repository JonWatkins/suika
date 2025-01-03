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
}
