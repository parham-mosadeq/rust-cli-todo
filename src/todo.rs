use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub task: String,
    pub done: bool,
    pub created_at: String,
    pub modified_at: String,
}

impl Todo {
    pub fn new(id: u32, task: String) -> Self {
        let now = Local::now().to_string();
        Todo {
            id,
            task,
            done: false,
            created_at: now.clone(),
            modified_at: now,
        }
    }
}

pub fn load_todos() -> Vec<Todo> {
    let data = fs::read_to_string("todos.json").unwrap_or("[]".into());
    serde_json::from_str(&data).unwrap()
}

pub fn save_todos(todos: &Vec<Todo>) {
    let data = serde_json::to_string_pretty(todos).unwrap();
    fs::write("todos.json", data).unwrap();
}
