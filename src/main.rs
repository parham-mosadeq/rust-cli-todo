use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: u32,
    task: String,
    done: bool,
    created_at: String,
    modified_at: String,
}

impl Todo {
    fn new(id: u32, task: String) -> Self {
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

fn load_todos() -> Vec<Todo> {
    let data = fs::read_to_string("todos.json").unwrap_or("[]".into());
    serde_json::from_str(&data).unwrap()
}

fn save_todos(todos: &Vec<Todo>) {
    let data = serde_json::to_string_pretty(todos).unwrap();
    fs::write("todos.json", data).unwrap();
}

fn main() {
    let mut todo_vec = load_todos();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: todo <command> [task]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
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
        "list" => {
            println!("Your todos:");
            for todo in &todo_vec {
                println!(
                    "[{}] {} - {}",
                    todo.id,
                    todo.task,
                    if todo.done { "done" } else { "pending" }
                );
            }
        }
        "find" => {
            if args.len() < 3 {
                println!("Please provide an id!");
            } else {
                let id: u32 = args[2].parse().unwrap();
                if let Some(todo) = todo_vec.iter().find(|t| t.id == id) {
                    println!("Found: {:?}", todo);
                } else {
                    println!("No todo found with id {}", id);
                }
            }
        }

        "mark" => {
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

        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
