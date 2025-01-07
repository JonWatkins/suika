use suika_json::JsonValue;
use std::collections::HashMap;

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
    values: HashMap<String, JsonValue>,
    parent: Option<Box<Context>>,
    macro_args: Option<HashMap<String, JsonValue>>,
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
        Self {
            values: HashMap::new(),
            parent: None,
            macro_args: None,
        }
    }

    /// Creates a new `Context` with a parent context.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_templates::context::Context;
    ///
    /// let parent = Context::new();
    /// let child = Context::with_parent(parent);
    /// assert!(child.get("key").is_none());
    /// ```
    pub fn with_parent(parent: Context) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(Box::new(parent)),
            macro_args: None,
        }
    }

    /// Creates a new `Context` with a parent context and macro arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_templates::context::Context;
    /// use std::collections::HashMap;
    ///
    /// let parent = Context::new();
    /// let child = Context::with_macro_args(parent, HashMap::new());
    /// assert!(child.get("key").is_none());
    /// ```
    pub fn with_macro_args(parent: Context, args: HashMap<String, JsonValue>) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(Box::new(parent)),
            macro_args: Some(args),
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
    pub fn insert<T>(&mut self, key: &str, value: T)
    where
        T: Into<JsonValue>,
    {
        self.values.insert(key.to_string(), value.into());
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
        // First check macro arguments
        if let Some(macro_args) = &self.macro_args {
            if let Some(value) = macro_args.get(key) {
                return Some(value);
            }
        }

        // Then check local values
        self.values.get(key).or_else(|| {
            // Finally check parent context
            self.parent.as_ref().and_then(|p| p.get(key))
        })
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, JsonValue> {
        self.values.iter()
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
