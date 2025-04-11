use random_str as random;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub category: String,
}

#[derive(Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Completed,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Completed => write!(f, "Completed"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

impl fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskPriority::Low => write!(f, "Low"),
            TaskPriority::Medium => write!(f, "Medium"),
            TaskPriority::High => write!(f, "High"),
        }
    }
}

impl TaskPriority {
    pub fn new(priority: &str) -> Option<Self> {
        match priority {
            "low" => Some(TaskPriority::Low),
            "medium" => Some(TaskPriority::Medium),
            "high" => Some(TaskPriority::High),
            _ => None,
        }
    }
}

impl Task {
    pub fn new(
        title: String,
        description: String,
        priority: TaskPriority,
        category: String,
    ) -> Self {
        Self {
            id: random::get_int(1, 100),
            title,
            description,
            priority,
            status: TaskStatus::Pending,
            category,
        }
    }

    pub fn task_info(&self) -> String {
        format!(
            "{:3} | {:<30} | {:<8} | {:<10} | {:<8}",
            self.id,
            self.title.replace('\n', " "),
            format!("{}", self.priority),
            self.category,
            self.status
        )
    }

    pub fn update_task(
        &mut self,
        title: String,
        description: String,
        priority: TaskPriority,
        category: String,
    ) {
        self.title = title;
        self.description = description;
        self.priority = priority;
        self.category = category;
    }

    pub fn mark_task_as_pending(&mut self) {
        self.status = TaskStatus::Pending;
    }

    pub fn mark_task_as_complete(&mut self) {
        self.status = TaskStatus::Completed;
    }
    
    pub fn is_completed(&self) -> bool {
        matches!(self.status, TaskStatus::Completed)
    }
}
