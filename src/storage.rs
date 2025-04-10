fn command_parser(command: &str) {
    match command.to_lowercase().as_str() {
        "add" => {},
        "list" => {},
        "complete" => {},
        "help" => {},
        "quit" | "exit" => {},
        _ => println!("Unknown command. Type 'help' for available commands.")
    }
}

fn configure_storage() {
    std::fs::create_dir("data").expect("Directory might exist already");
}