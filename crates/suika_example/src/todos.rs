use std::sync::{Arc, RwLock};
use suika::json::JsonValue;

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub slug: String,
    pub content: String,
}

impl Into<JsonValue> for Todo {
    fn into(self) -> JsonValue {
        JsonValue::Object(vec![
            ("id".to_string(), JsonValue::Number(self.id as f64)),
            ("title".to_string(), JsonValue::String(self.title)),
            ("slug".to_string(), JsonValue::String(self.slug)),
            ("content".to_string(), JsonValue::String(self.content)),
        ])
    }
}

pub struct TodoStore {
    todos: Arc<RwLock<Vec<Todo>>>,
    next_id: Arc<RwLock<usize>>,
}

impl TodoStore {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(RwLock::new(Vec::new())),
            next_id: Arc::new(RwLock::new(1)),
        }
    }

    pub fn add_todo<T: Into<String>, U: Into<String>>(&self, title: T, content: U) -> Todo {
        let mut todos = self.todos.write().unwrap();
        let mut next_id = self.next_id.write().unwrap();

        let title: String = title.into();
        let todo = Todo {
            id: *next_id,
            title: title.clone(),
            slug: Self::generate_slug(&title),
            content: content.into(),
        };

        todos.push(todo.clone());
        *next_id += 1;

        todo
    }

    pub fn to_json(&self) -> JsonValue {
        let todos = self.todos.read().unwrap();
        JsonValue::Array(todos.iter().map(|todo| todo.clone().into()).collect())
    }

    fn generate_slug(title: &str) -> String {
        title.to_lowercase().replace(" ", "-")
    }
}
