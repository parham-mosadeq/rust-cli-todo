# Rust CLI Todo App

A simple command-line Todo application written in Rust. Allows you to **add**, **list**, **find**, and **mark** tasks, with persistent storage in a JSON file.

## Commands

```bash
# Add a new task
cargo run -- add "Task description"

# List all tasks
cargo run -- list

# Find a task by ID
cargo run -- find <id>   # Example: 1, 2, 3...

# Mark a task as done or pending
cargo run -- mark <id> <true|false>   # Example: cargo run -- mark 1 true
```
