use crate::{Task, TaskPriority};
use std::io::{self, Write};

pub fn add_tasks(task_list: &mut Vec<Task>, words: Vec<&str>) {
    if words.len() > 1 {
        println!("Invalid 'add' command! Use 'add' for task prompts.");
        return;
    }

    // Get title with validation loop
    let mut title = String::new();
    loop {
        print!("Title: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut title)
            .expect("Failed to read line");
        let title = title.trim().to_string();
        if !title.is_empty() {
            break;
        }
        println!("Title cannot be empty. Please try again.");
    }

    // Get description with validation loop
    let mut description = String::new();
    print!("Description: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");
    description = description.trim().to_string();

    // Get priority with validation loop
    let mut priority = TaskPriority::Low;
    loop {
        print!("Priority (low/medium/high): ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut priority_input = String::new();
        io::stdin()
            .read_line(&mut priority_input)
            .expect("Failed to read line");
        let priority_input = priority_input.trim().to_lowercase();

        match TaskPriority::new(&priority_input) {
            Some(p) => {
                priority = p;
                break;
            }
            None => {
                if priority_input.is_empty() {
                    println!("Using default priority: low");
                    break;
                } else {
                    println!("Invalid priority. Please enter 'low', 'medium', or 'high'.");
                }
            }
        }
    }

    // Get category with validation loop
    let mut category = String::new();
    print!("Category: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut category)
        .expect("Failed to read line");
    category = category.trim().to_string();

    task_list.push(Task::new(title, description, priority, category));
    println!("Task added successfully!");
}

pub fn list_tasks(task_list: &[Task]) {
    if task_list.is_empty() {
        println!("No tasks added yet.");
    } else {
        println!(
            "{:3} | {:<30} | {:<8} | {:<10} | {:<8}",
            "ID", "Title", "Priority", "Category", "Status"
        );
        println!("{}", "-".repeat(70)); // Add a separator line
        task_list
            .iter()
            .for_each(|task| println!("{}", task.task_info()))
    }
}

pub fn filter_tasks(task_list: &[Task]) {
    println!("Filter tasks by category or status:");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice = choice.trim().to_string();

    if choice == "category" {
        println!("Filter tasks by category:");
        let mut category = String::new();
        print!("Category: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut category)
            .expect("Failed to read line");
        category = category.trim().to_string();

        let filtered_tasks = task_list
            .iter()
            .filter(|task| task.category.to_lowercase() == category.to_lowercase())
            .collect::<Vec<&Task>>();

        if filtered_tasks.is_empty() {
            println!("No tasks found in category '{}'", category);
        } else {
            println!(
                "{:3} | {:<30} | {:<8} | {:<10} | {:<8}",
                "ID", "Title", "Priority", "Category", "Status"
            );
            println!("{}", "-".repeat(70)); // Add a separator line
            filtered_tasks
                .iter()
                .for_each(|task| println!("{}", task.task_info()));
        }
    } else if choice == "status" {
        println!("Filter tasks by status:");
        let mut status = String::new();
        print!("Status: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut status)
            .expect("Failed to read line");
        status = status.trim().to_string();

        let filtered_tasks = task_list
            .iter()
            .filter(|task| task.status.to_string().to_lowercase() == status.to_lowercase())
            .collect::<Vec<&Task>>();

        if filtered_tasks.is_empty() {
            println!("No tasks found with the specified status.");
        } else {
            println!(
                "{:3} | {:<30} | {:<8} | {:<10} | {:<8}",
                "ID", "Title", "Priority", "Category", "Status"
            );
            println!("{}", "-".repeat(70)); // Add a separator line
            filtered_tasks
                .iter()
                .for_each(|task| println!("{}", task.task_info()));
        }
    } else {
        println!("Invalid choice. Use 'category' or 'status'.");
    }
}
