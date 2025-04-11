use std::io;

use models::Task;

mod command_processing;
mod models;
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

        // Check for "help" command
        if input.eq_ignore_ascii_case("help") {
            ui::available_commands();
            continue;
        }

        // Split input into words
        let words = input.split_whitespace().collect::<Vec<&str>>();

        if words.is_empty() {
            println!("Invalid command!");
            continue;
        }

        // Match command
        match words[0].to_lowercase().as_str() {
            "add" => command_processing::add_tasks(&mut task_list, words),
            "list" => command_processing::list_tasks(&task_list),
            "filter" => command_processing::filter_tasks(&task_list),
            "update" => command_processing::update_task(&mut task_list),
            "complete" => command_processing::complete_task(&mut task_list),
            "undo" => command_processing::undo_task(&mut task_list),
            "delete" => command_processing::delete_task(&mut task_list),
            "stats" => command_processing::show_stats(&task_list),
            _ => println!("Unknown command. Type 'help' for available commands."),
        }
    }

    println!("Goodbye!");
}
