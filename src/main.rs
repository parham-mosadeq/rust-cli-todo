mod todo;
use std::env;
use todo::{add, find, list, load_todos, mark};

fn main() {
    let mut todo_vec = load_todos();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: todo <command> [task]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => add(&args, &mut todo_vec),
        "list" => list(&todo_vec),
        "find" => find(&args, &todo_vec),
        "mark" => mark(&args, &mut todo_vec),
        _ => println!("Unknown command: {}", command),
    }
}
