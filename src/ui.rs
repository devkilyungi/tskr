pub fn welcome_message() {
    // Display initial instructions to the user
    println!("tskr: Get things done! v.1.0");
    available_commands();
    println!("\nLet's get started")
}

pub fn available_commands() {
    println!("\nAvailable commands:");
    println!("  add      - Add a new task");
    println!("  list     - List all tasks");
    println!("  filter   - Filter tasks by category or status");
    println!("  update   - Update an existing task");
    println!("  complete - Mark a task as complete");
    println!("  delete   - Delete a task");
    println!("  stats    - Show task statistics");
    println!("  help     - Show this help message");
    println!("  exit     - Exit the program");
    println!("  quit     - Exit the program");
}
