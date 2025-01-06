use std::collections::HashMap;
use suika_json::JsonValue;

/// A context for storing key-value pairs where the values are JSON values.
///
/// The `Context` struct is used to store key-value pairs where the values are of type `JsonValue`.
/// This is useful for templating engines and other applications where dynamic data needs to be
/// stored and retrieved.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use suika_json::JsonValue;
/// use suika_templates::context::Context;
///
/// let mut context = Context::new();
/// context.insert("name", "John");
/// context.insert("age", 30);
///
/// if let Some(name) = context.get("name") {
///     assert_eq!(name, &JsonValue::String("John".to_string()));
/// }
///
/// if let Some(age) = context.get("age") {
///     assert_eq!(age, &JsonValue::Number(30.0));
/// }
/// ```
#[derive(Clone)]
pub struct Context {
    data: HashMap<String, JsonValue>,
}

impl Context {
    /// Creates a new, empty `Context`.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_templates::context::Context;
    ///
    /// let context = Context::new();
    /// assert!(context.get("key").is_none());
    /// ```
    pub fn new() -> Self {
        Context {
            data: HashMap::new(),
        }
    }

    /// Inserts a key-value pair into the context.
    ///
    /// If the context did not have this key present, `None` is returned.
    /// If the context did have this key present, the value is updated, and the old value is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_json::JsonValue;
    /// use suika_templates::context::Context;
    ///
    /// let mut context = Context::new();
    /// context.insert("name", "John");
    /// assert_eq!(context.get("name"), Some(&JsonValue::String("John".to_string())));
    /// ```
    pub fn insert<T: Into<JsonValue>>(&mut self, key: &str, value: T) {
        self.data.insert(key.to_string(), value.into());
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// If the key is not found, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_json::JsonValue;
    /// use suika_templates::context::Context;
    ///
    /// let mut context = Context::new();
    /// context.insert("name", "John");
    /// assert_eq!(context.get("name"), Some(&JsonValue::String("John".to_string())));
    /// assert!(context.get("age").is_none());
    /// ```
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        self.data.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use suika_json::JsonValue;
    use std::collections::HashMap;

    #[test]
    fn test_new() {
        let context = Context::new();
        assert!(context.get("key").is_none());
    }

    #[test]
    fn test_insert_and_get() {
        let mut context = Context::new();
        context.insert("name", "John");
        assert_eq!(context.get("name"), Some(&JsonValue::String("John".to_string())));

        context.insert("age", 30);
        assert_eq!(context.get("age"), Some(&JsonValue::Number(30.0)));

        context.insert("is_student", true);
        assert_eq!(context.get("is_student"), Some(&JsonValue::Boolean(true)));
    }

    #[test]
    fn test_insert_overwrite() {
        let mut context = Context::new();
        context.insert("name", "John");
        assert_eq!(context.get("name"), Some(&JsonValue::String("John".to_string())));

        context.insert("name", "Jane");
        assert_eq!(context.get("name"), Some(&JsonValue::String("Jane".to_string())));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let context = Context::new();
        assert!(context.get("nonexistent").is_none());
    }

    #[test]
    fn test_insert_complex_types() {
        let mut context = Context::new();

        // Insert a vector
        context.insert("numbers", vec![1, 2, 3]);
        assert_eq!(
            context.get("numbers"),
            Some(&JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0)
            ]))
        );

        // Insert a hashmap
        let mut map = HashMap::new();
        map.insert("key1".to_string(), JsonValue::String("value1".to_string()));
        map.insert("key2".to_string(), JsonValue::Number(42.0));
        context.insert("map", map.clone());
        assert_eq!(context.get("map"), Some(&JsonValue::Object(map.into_iter().collect())));
    }
}
