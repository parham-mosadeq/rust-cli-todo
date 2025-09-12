mod todo;
use std::env;
use todo::{Todo, load_todos, save_todos};

fn add(args: Vec<String>, mut todo_vec: Vec<Todo>) {
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

fn list(todo_vec: Vec<Todo>) {
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

fn find(args: Vec<String>, todo_vec: Vec<Todo>) {
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

fn mark(args: Vec<String>, mut todo_vec: Vec<Todo>) {
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

fn main() {
    let todo_vec = load_todos();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: todo <command> [task]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            add(args, todo_vec)
            // if args.len() < 3 {
            //     println!("Please provide a task!");
            // } else {
            //     let task = &args[2];
            //     let id = (todo_vec.len() as u32) + 1;
            //     let new_task = Todo::new(id, task.to_string());
            //     todo_vec.push(new_task);
            //     save_todos(&todo_vec);
            //     println!("Task added!");
            // }
        }
        "list" => {
            // println!("Your todos:");
            // for todo in &todo_vec {
            //     println!(
            //         "[{}] {} - {}",
            //         todo.id,
            //         todo.task,
            //         if todo.done { "done" } else { "pending" }
            //     );
            // }
            list(todo_vec)
        }
        "find" => {
            find(args, todo_vec);
            // if args.len() < 3 {
            //     println!("Please provide an id!");
            // } else {
            //     let id: u32 = args[2].parse().unwrap();
            //     if let Some(todo) = todo_vec.iter().find(|t| t.id == id) {
            //         println!("Found: {:?}", todo);
            //     } else {
            //         println!("No todo found with id {}", id);
            //     }
            // }
        }

        "mark" => {
            mark(args, todo_vec);
            // if args.len() < 4 {
            //     println!("Usage: todo mark <id> <true|false>");
            // } else {
            //     let id: u32 = args[2].parse().unwrap();
            //     let is_done: bool = args[3].parse().unwrap_or(false);

            //     if let Some(todo) = todo_vec.iter_mut().find(|t| t.id == id) {
            //         todo.done = is_done;
            //         todo.modified_at = chrono::Local::now().to_string();
            //         save_todos(&todo_vec); // don’t forget to persist
            //         println!("Todo {} marked as {}", id, is_done);
            //     } else {
            //         println!("No todo found with id {}", id);
            //     }
            // }
        }

        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
