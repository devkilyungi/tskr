use crate::models::{Task, TaskPriority};
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
        println!("Filter tasks by status (pending/completed):");
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

pub fn update_task(task_list: &mut [Task]) {
    println!("Update task by ID:");
    let mut id = String::new();
    print!("ID: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut id).expect("Failed to read line");
    let id = id.trim().parse::<i32>().expect("Invalid ID");

    if let Some(task_index) = task_list.iter().position(|task| task.id == id) {
        println!("Enter new task details:");
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

        task_list[task_index].update_task(title, description, priority, category);
        println!("Task updated successfully!");
    } else {
        println!("Task not found!");
    }
}

pub fn complete_task(task_list: &mut [Task]) {
    println!("Complete Task by ID:");
    let mut id_input = String::new();
    print!("ID: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut id_input)
        .expect("Failed to read line");
    let id_input = id_input.trim().parse::<i32>().expect("Invalid ID");

    if let Some(task) = task_list.iter_mut().find(|t| t.id == id_input) {
        task.mark_task_as_complete();
        println!("Task completed successfully!");
    } else {
        println!("Task not found!");
    }
}

pub fn undo_task(task_list: &mut [Task]) {
    println!("Undo Task by ID:");
    let mut id_input = String::new();
    print!("ID: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut id_input)
        .expect("Failed to read line");
    let id_input = id_input.trim().parse::<i32>().expect("Invalid ID");

    if let Some(task) = task_list.iter_mut().find(|t| t.id == id_input) {
        task.mark_task_as_pending();
        println!("Task marked as 'Pending' successfully!");
    } else {
        println!("Task not found!");
    }
}

pub fn delete_task(task_list: &mut Vec<Task>) {
    println!("Delete Task by ID:");
    let mut id_input = String::new();
    print!("ID: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut id_input)
        .expect("Failed to read line");
    let id_input = id_input.trim().parse::<i32>().expect("Invalid ID");

    if let Some(task_index) = task_list.iter().position(|t| t.id == id_input) {
        task_list.remove(task_index);
        println!("Task deleted successfully!");
    } else {
        println!("Task not found!");
    }
}

pub fn show_stats(task_list: &[Task]) {
    let completed_tasks = task_list.iter().filter(|t| t.is_completed()).count();
    let pending_tasks = task_list.iter().filter(|t| !t.is_completed()).count();

    let completed_tasks_percent = if task_list.is_empty() {
        0.0
    } else {
        (completed_tasks as f32 / task_list.len() as f32) * 100.0
    };
    let pending_tasks_percent = if task_list.is_empty() {
        0.0
    } else {
        (pending_tasks as f32 / task_list.len() as f32) * 100.0
    };

    println!("Total tasks: {}", task_list.len());
    println!(
        "Completed Tasks: {} ({}%)",
        completed_tasks, completed_tasks_percent
    );
    println!(
        "Pending Tasks: {} ({}%)",
        pending_tasks, pending_tasks_percent
    );
    println!("Task by category: ");
    if task_list.is_empty() {
        println!("No tasks found!");
    } else {
        task_list.iter().for_each(|task| {
            println!("{}: {}", task.category, task.title);
        });
    }
    println!("Task by priority: ");
    if task_list.is_empty() {
        println!("No tasks found!");
    } else {
        task_list.iter().for_each(|task| {
            println!("{}: {}", task.priority, task.title);
        });
    }
}
