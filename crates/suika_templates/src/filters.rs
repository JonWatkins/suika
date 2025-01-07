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
            let result = filter(input)?;
            Ok(result.into_json())
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
        self.register("upper", |value: String| Ok(value.to_uppercase()));
        self.register("lower", |value: String| Ok(value.to_lowercase()));
        self.register("length", |value: Vec<JsonValue>| Ok(value.len() as f64));
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
}
