use std::collections::HashMap;
use suika_json::JsonValue;

pub type FilterFn = fn(JsonValue, Vec<JsonValue>) -> Result<JsonValue, String>;

#[derive(Debug, Clone)]
pub struct FilterRegistry {
    filters: HashMap<String, FilterFn>,
}

impl FilterRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            filters: HashMap::new(),
        };
        registry.register_default_filters();
        registry
    }

    pub fn register(&mut self, name: &str, filter: FilterFn) {
        self.filters.insert(name.to_string(), filter);
    }

    pub fn get(&self, name: &str) -> Option<&FilterFn> {
        self.filters.get(name)
    }

    fn register_default_filters(&mut self) {
        self.register("upper", |value, _args| match value {
            JsonValue::String(s) => Ok(JsonValue::String(s.to_uppercase())),
            _ => Err("upper filter expects a string".to_string()),
        });

        self.register("lower", |value, _args| match value {
            JsonValue::String(s) => Ok(JsonValue::String(s.to_lowercase())),
            _ => Err("lower filter expects a string".to_string()),
        });

        self.register("length", |value, _args| match value {
            JsonValue::Array(arr) => Ok(JsonValue::Number(arr.len() as f64)),
            JsonValue::String(s) => Ok(JsonValue::Number(s.len() as f64)),
            _ => Err("length filter expects an array or string".to_string()),
        });
    }
} 
