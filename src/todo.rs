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

pub fn add(args: &Vec<String>, todo_vec: &mut Vec<Todo>) {
    if args.len() < 3 {
        println!("Please provide a task!");
    } else {
        let task = &args[2];
        let id = (todo_vec.len() as u32) + 1;
        let new_task = Todo::new(id, task.to_string());
        todo_vec.push(new_task);
        save_todos(&todo_vec);
        println!("Task added!");
    }
}

pub fn list(todo_vec: &Vec<Todo>) {
    println!("Your todos:");
    for todo in todo_vec {
        println!(
            "[{}] {} - {}",
            todo.id,
            todo.task,
            if todo.done { "done" } else { "pending" }
        );
    }
}

pub fn find(args: &Vec<String>, todo_vec: &Vec<Todo>) {
    if args.len() < 3 {
        println!("Please provide an id!");
    } else {
        let id: u32 = args[2].parse().unwrap();
        if let Some(todo) = todo_vec.iter().find(|t| t.id == id) {
            println!("Found: {:#?}", todo);
        } else {
            println!("No todo found with id {}", id);
        }
    }
}

pub fn mark(args: &Vec<String>, todo_vec: &mut Vec<Todo>) {
    if args.len() < 4 {
        println!("Usage: todo mark <id> <true|false>");
    } else {
        let id: u32 = args[2].parse().unwrap();
        let is_done: bool = args[3].parse().unwrap_or(false);

        if let Some(todo) = todo_vec.iter_mut().find(|t| t.id == id) {
            todo.done = is_done;
            todo.modified_at = chrono::Local::now().to_string();
            save_todos(&todo_vec); // don’t forget to persist
            println!("Todo {} marked as {}", id, is_done);
        } else {
            println!("No todo found with id {}", id);
        }
    }
}
