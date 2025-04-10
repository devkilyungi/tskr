use std::io::{self, Write};

use models::{Task, TaskPriority};
use ui::available_commands;

mod error;
mod models;
mod storage;
mod ui;

fn main() {
    ui::welcome_message();

    let mut task_list: Vec<Task> = Vec::new();

    loop {
        println!("\nEnter a command: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        // Check for "quit" | "exit" command
        if input.eq_ignore_ascii_case("exit") | input.eq_ignore_ascii_case("quit") {
            break;
        }

        let words = input.split_whitespace().collect::<Vec<&str>>();

        if words.is_empty() {
            println!("Invalid command!");
            continue;
        }

        match words[0].to_lowercase().as_str() {
            "add" => {
                if words.len() > 1 {
                    println!("Invalid 'add' command! Use 'add' for task prompts.");
                    continue;
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
                                println!(
                                    "Invalid priority. Please enter 'low', 'medium', or 'high'."
                                );
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
            "list" => {
                if task_list.is_empty() {
                    println!("No tasks added yet.");
                } else {
                    println!("{:3} | {:<30} | {:<8} | {:<10} | {:<8}", "ID", "Title", "Priority", "Category", "Status");
                    println!("{}", "-".repeat(70)); // Add a separator line
                    task_list
                        .iter()
                        .for_each(|task| println!("{}", task.task_info()))
                }
            }
            "filter" => {}
            "update" => {}
            "complete" => {}
            "delete" => {}
            "stats" => {}
            "help" => {
                available_commands();
            }
            _ => println!("Unknown command. Type 'help' for available commands."),
        }
    }

    println!("Goodbye!");
}
