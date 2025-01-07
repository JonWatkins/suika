use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use suika_json::JsonValue;

/// Trait for converting JSON values into specific Rust types for filter input.
///
/// This trait allows filters to work with strongly-typed values instead of raw JsonValue.
///
/// # Examples
///
/// ```
/// # use suika_json::JsonValue;
/// # use suika_templates::filters::FromJsonValue;
/// #
/// struct MyType(String);
///
/// impl FromJsonValue for MyType {
///     fn from_json(value: JsonValue) -> Result<Self, String> {
///         match value {
///             JsonValue::String(s) => Ok(MyType(s)),
///             _ => Err("Expected string value".to_string()),
///         }
///     }
/// }
/// ```
pub trait FromJsonValue: Sized {
    fn from_json(value: JsonValue) -> Result<Self, String>;
}

/// Trait for converting filter output values into JSON values.
///
/// This trait allows filter results to be converted back into JsonValue for template rendering.
///
/// # Examples
///
/// ```
/// # use suika_json::JsonValue;
/// # use suika_templates::filters::IntoJsonValue;
/// #
/// struct MyType(String);
///
/// impl IntoJsonValue for MyType {
///     fn into_json(self) -> JsonValue {
///         JsonValue::String(self.0)
///     }
/// }
/// ```
pub trait IntoJsonValue {
    fn into_json(self) -> JsonValue;
}

// Implement conversion traits for common types
impl FromJsonValue for String {
    fn from_json(value: JsonValue) -> Result<Self, String> {
        match value {
            JsonValue::String(s) => Ok(s),
            _ => Err("Expected string value".to_string()),
        }
    }
}

impl FromJsonValue for f64 {
    fn from_json(value: JsonValue) -> Result<Self, String> {
        match value {
            JsonValue::Number(n) => Ok(n),
            _ => Err("Expected number value".to_string()),
        }
    }
}

impl FromJsonValue for Vec<JsonValue> {
    fn from_json(value: JsonValue) -> Result<Self, String> {
        match value {
            JsonValue::Array(arr) => Ok(arr),
            _ => Err("Expected array value".to_string()),
        }
    }
}

impl IntoJsonValue for String {
    fn into_json(self) -> JsonValue {
        JsonValue::String(self)
    }
}

impl IntoJsonValue for f64 {
    fn into_json(self) -> JsonValue {
        JsonValue::Number(self)
    }
}

impl FromJsonValue for JsonValue {
    fn from_json(value: JsonValue) -> Result<Self, String> {
        Ok(value)
    }
}

impl IntoJsonValue for JsonValue {
    fn into_json(self) -> JsonValue {
        self
    }
}

/// A registry for storing and managing template filters.
///
/// # Examples
///
/// ```
/// # use suika_templates::filters::FilterRegistry;
/// #
/// let mut registry = FilterRegistry::new();
/// registry.register("upper", |value: String| Ok(value.to_uppercase()));
/// ```
pub struct FilterRegistry {
    filters: HashMap<
        String,
        Arc<dyn Fn(JsonValue, Vec<JsonValue>) -> Result<JsonValue, String> + Send + Sync>,
    >,
}

impl FilterRegistry {
    /// Creates a new FilterRegistry with default filters.
    pub fn new() -> Self {
        let mut registry = Self {
            filters: HashMap::new(),
        };
        registry.register_default_filters();
        registry
    }

    /// Registers a new filter function with type-safe conversion.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The filter function type
    /// * `T` - The input type that implements FromJsonValue
    /// * `R` - The return type that implements IntoJsonValue
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the filter to register
    /// * `filter` - The filter function that transforms values
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_json::JsonValue;
    /// use suika_templates::filters::FilterRegistry;
    ///
    /// let mut registry = FilterRegistry::new();
    /// registry.register("upper", |value: String| Ok(value.to_uppercase()));
    ///
    /// let filter = registry.get("upper").unwrap();
    /// let result = filter(JsonValue::String("hello".to_string()), vec![]).unwrap();
    /// assert_eq!(result, JsonValue::String("HELLO".to_string()));
    /// ```
    pub fn register<F, T, R>(&mut self, name: &str, filter: F)
    where
        F: Fn(T) -> Result<R, String> + Send + Sync + 'static,
        T: FromJsonValue,
        R: IntoJsonValue,
    {
        let filter = Arc::new(move |value: JsonValue, _args: Vec<JsonValue>| {
            let input = T::from_json(value)?;
            let output = filter(input)?;
            Ok(output.into_json())
        });
        self.filters.insert(name.to_string(), filter);
    }

    /// Gets a filter function by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the filter to retrieve
    ///
    /// # Returns
    ///
    /// An Option containing a reference to the filter function if found
    pub fn get(
        &self,
        name: &str,
    ) -> Option<&(dyn Fn(JsonValue, Vec<JsonValue>) -> Result<JsonValue, String> + Send + Sync)>
    {
        self.filters.get(name).map(|f| f.as_ref())
    }

    /// Registers the default set of filters (upper, lower, length).
    fn register_default_filters(&mut self) {
        // String transformations
        self.register("upper", |s: String| Ok(s.to_uppercase()));
        self.register("lower", |s: String| Ok(s.to_lowercase()));

        self.register("capitalize", |s: String| {
            let mut chars = s.chars();
            match chars.next() {
                None => Ok(String::new()),
                Some(first) => Ok(first.to_uppercase().chain(chars).collect()),
            }
        });

        self.register("trim", |s: String| -> Result<String, String> {
            Ok(s.trim().to_string())
        });

        self.register("reverse", |s: String| -> Result<String, String> {
            Ok(s.chars().rev().collect())
        });

        // Collection operations
        self.register("length", |value: JsonValue| match value {
            JsonValue::Array(arr) => Ok(JsonValue::Number(arr.len() as f64)),
            JsonValue::String(s) => Ok(JsonValue::Number(s.len() as f64)),
            _ => Err("length filter only works on arrays and strings".to_string()),
        });

        self.register("first", |value: JsonValue| match value {
            JsonValue::Array(arr) => arr
                .first()
                .cloned()
                .ok_or_else(|| "Array is empty".to_string()),
            JsonValue::String(s) => Ok(JsonValue::String(
                s.chars().next().map(|c| c.to_string()).unwrap_or_default(),
            )),
            _ => Err("first filter only works on arrays and strings".to_string()),
        });

        self.register("last", |value: JsonValue| match value {
            JsonValue::Array(arr) => arr
                .last()
                .cloned()
                .ok_or_else(|| "Array is empty".to_string()),
            JsonValue::String(s) => Ok(JsonValue::String(
                s.chars().last().map(|c| c.to_string()).unwrap_or_default(),
            )),
            _ => Err("last filter only works on arrays and strings".to_string()),
        });

        // Array operations
        self.register("join", |value: JsonValue| match value {
            JsonValue::Array(arr) => Ok(JsonValue::String(
                arr.iter()
                    .map(|x| match x {
                        JsonValue::String(s) => s.clone(),
                        _ => x.to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
            )),
            _ => Err("join filter only works on arrays".to_string()),
        });

        self.register(
            "json_encode",
            |value: JsonValue| -> Result<String, String> { Ok(value.to_string()) },
        );
    }
}

impl fmt::Debug for FilterRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FilterRegistry")
            .field("filters", &self.filters.keys())
            .finish()
    }
}

impl Clone for FilterRegistry {
    fn clone(&self) -> Self {
        let mut new_registry = Self {
            filters: HashMap::new(),
        };
        for (name, filter) in &self.filters {
            let filter = filter.clone();
            new_registry
                .filters
                .insert(name.clone(), Arc::new(move |v, args| filter(v, args)));
        }
        new_registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_filter() {
        let mut registry = FilterRegistry::new();

        registry.register("greet", |name: String| Ok(format!("Hello, {}!", name)));

        let filter = registry.get("greet").unwrap();
        let result = filter(JsonValue::String("World".to_string()), vec![]).unwrap();
        assert_eq!(result, JsonValue::String("Hello, World!".to_string()));
    }

    #[test]
    fn test_register_number_filter() {
        let mut registry = FilterRegistry::new();

        registry.register("add_one", |num: f64| Ok(num + 1.0));

        let filter = registry.get("add_one").unwrap();
        let result = filter(JsonValue::Number(41.0), vec![]).unwrap();
        assert_eq!(result, JsonValue::Number(42.0));
    }

    #[test]
    fn test_string_filters() {
        let registry = FilterRegistry::new();

        // Test upper
        let filter = registry.get("upper").unwrap();
        let result = filter(JsonValue::String("hello".to_string()), vec![]).unwrap();
        assert_eq!(result, JsonValue::String("HELLO".to_string()));

        // Test lower
        let filter = registry.get("lower").unwrap();
        let result = filter(JsonValue::String("HELLO".to_string()), vec![]).unwrap();
        assert_eq!(result, JsonValue::String("hello".to_string()));

        // Test capitalize
        let filter = registry.get("capitalize").unwrap();
        let result = filter(JsonValue::String("hello world".to_string()), vec![]).unwrap();
        assert_eq!(result, JsonValue::String("Hello world".to_string()));

        // Test trim
        let filter = registry.get("trim").unwrap();
        let result = filter(JsonValue::String("  hello  ".to_string()), vec![]).unwrap();
        assert_eq!(result, JsonValue::String("hello".to_string()));

        // Test reverse
        let filter = registry.get("reverse").unwrap();
        let result = filter(JsonValue::String("hello".to_string()), vec![]).unwrap();
        assert_eq!(result, JsonValue::String("olleh".to_string()));
    }

    #[test]
    fn test_length_filter() {
        let registry = FilterRegistry::new();
        let filter = registry.get("length").unwrap();

        // Test array length
        let arr = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]);
        let result = filter(arr, vec![]).unwrap();
        assert_eq!(result, JsonValue::Number(3.0));

        // Test string length
        let result = filter(JsonValue::String("hello".to_string()), vec![]).unwrap();
        assert_eq!(result, JsonValue::Number(5.0));

        // Test invalid type
        let result = filter(JsonValue::Number(42.0), vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_first_last_filters() {
        let registry = FilterRegistry::new();

        // Test first filter
        let first_filter = registry.get("first").unwrap();
        let arr = JsonValue::Array(vec![
            JsonValue::String("one".to_string()),
            JsonValue::String("two".to_string()),
        ]);
        let result = first_filter(arr.clone(), vec![]).unwrap();
        assert_eq!(result, JsonValue::String("one".to_string()));

        // Test last filter
        let last_filter = registry.get("last").unwrap();
        let result = last_filter(arr, vec![]).unwrap();
        assert_eq!(result, JsonValue::String("two".to_string()));

        // Test first/last on string
        let result = first_filter(JsonValue::String("hello".to_string()), vec![]).unwrap();
        assert_eq!(result, JsonValue::String("h".to_string()));
        let result = last_filter(JsonValue::String("hello".to_string()), vec![]).unwrap();
        assert_eq!(result, JsonValue::String("o".to_string()));
    }

    #[test]
    fn test_join_filter() {
        let registry = FilterRegistry::new();
        let filter = registry.get("join").unwrap();

        let arr = JsonValue::Array(vec![
            JsonValue::String("one".to_string()),
            JsonValue::String("two".to_string()),
            JsonValue::String("three".to_string()),
        ]);
        let result = filter(arr, vec![]).unwrap();
        assert_eq!(result, JsonValue::String("one, two, three".to_string()));

        // Test invalid type
        let result = filter(JsonValue::String("not an array".to_string()), vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_json_encode_filter() {
        let registry = FilterRegistry::new();
        let filter = registry.get("json_encode").unwrap();

        // Test object
        let obj = JsonValue::Object(vec![
            ("name".to_string(), JsonValue::String("John".to_string())),
            ("age".to_string(), JsonValue::Number(30.0)),
        ]);
        let result = filter(obj, vec![]).unwrap();
        assert_eq!(
            result,
            JsonValue::String(r#"{"name":"John","age":30}"#.to_string())
        );

        // Test array
        let arr = JsonValue::Array(vec![
            JsonValue::String("one".to_string()),
            JsonValue::Number(2.0),
        ]);
        let result = filter(arr, vec![]).unwrap();
        assert_eq!(result, JsonValue::String(r#"["one",2]"#.to_string()));
    }
}
