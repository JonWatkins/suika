use std::collections::HashMap;
use std::fmt;

/// Represents a value in a template context.
///
/// # Examples
///
/// ```
/// use suika_templates::TemplateValue;
///
/// let string_value = TemplateValue::String("Hello".to_string());
/// assert_eq!(string_value.to_string(), "Hello");
///
/// let boolean_value = TemplateValue::Boolean(true);
/// assert_eq!(boolean_value.to_string(), "true");
///
/// let array_value = TemplateValue::Array(vec![
///     TemplateValue::String("One".to_string()),
///     TemplateValue::String("Two".to_string())
/// ]);
/// assert_eq!(array_value.to_string(), "[One, Two]");
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum TemplateValue {
    String(String),
    Boolean(bool),
    Array(Vec<TemplateValue>),
    Object(HashMap<String, TemplateValue>),
}

impl fmt::Display for TemplateValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateValue::String(s) => write!(f, "{}", s),
            TemplateValue::Boolean(b) => write!(f, "{}", b),
            TemplateValue::Array(arr) => {
                let entries: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", entries.join(", "))
            },
            TemplateValue::Object(obj) => {
                let entries: Vec<String> = obj.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(f, "{{{}}}", entries.join(", "))
            }
        }
    }
}
