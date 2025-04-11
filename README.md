# tskr - Command-Line Task Manager

A simple command-line task management application built in Rust. Track your tasks with priorities, categories, and completion status.

## Features

- Create, read, update, and delete tasks
- Categorize tasks (work, personal, study, etc.)
- Set priority levels (low, medium, high)
- Mark tasks as complete or pending
- Filter tasks by category or status
- View task statistics
- Persistent storage using JSON

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/devkilyungi/tskr.git
   cd tskr
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the application:
   ```
   cargo run
   ```

## Usage

Available commands:

- `add` - Add a new task
- `list` - List all tasks
- `filter` - Filter tasks by category or status
- `update` - Update an existing task
- `complete` - Mark a task as complete
- `undo` - Mark a complete task as pending
- `delete` - Delete a task
- `stats` - Show task statistics
- `help` - Show help message
- `exit` or `quit` - Exit the application

## Example

```
tskr: Get things done! v.1.0

Available commands:
  add      - Add a new task
  list     - List all tasks
  filter   - Filter tasks by category or status
  update   - Update an existing task
  complete - Mark a task as complete
  undo     - Mark complete task as pending
  delete   - Delete a task
  stats    - Show task statistics
  help     - Show this help message
  exit     - Exit the program
  quit     - Exit the program

Let's get started

Enter a command: 
add
Title: Complete Rust project
Description: Finish the command-line task manager
Priority (low/medium/high): high
Category: coding
Task added successfully!
```

## License

MIT

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.