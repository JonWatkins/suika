use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub slug: String,
    pub content: String,
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

    pub fn add_todo(&self, title: String, content: String) -> Todo {
        let mut todos = self.todos.write().unwrap();
        let mut next_id = self.next_id.write().unwrap();

        let todo = Todo {
            id: *next_id,
            title: title.clone(),
            slug: Self::generate_slug(&title),
            content,
        };

        todos.push(todo.clone());
        *next_id += 1;

        todo
    }

    pub fn get_todos(&self) -> Vec<Todo> {
        let todos = self.todos.read().unwrap();
        todos.clone()
    }

    fn generate_slug(title: &str) -> String {
        title.to_lowercase().replace(" ", "-")
    }
}
