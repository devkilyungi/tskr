use std::io;

use crate::models::Task;

pub fn save_to_json(tasks: &[Task]) -> io::Result<bool> {
    let json = serde_json::to_string_pretty(tasks)?;
    std::fs::write("tasks.json", json)?;
    Ok(true)
}

pub fn load_from_json(path: &str) -> io::Result<Vec<Task>> {
    let json = std::fs::read_to_string(path)?;
    let tasks = serde_json::from_str(&json)?;
    Ok(tasks)
}
