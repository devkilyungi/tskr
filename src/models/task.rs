use random_str as random;

enum TaskStatus {
    Pending, Completed
}

enum TaskPriority {
    Low, Medium, High
}

pub struct Task {
    id: String, 
    title: String,
    description: Option<String>,
    priority: TaskPriority,
    status: TaskStatus,
    category: String,
}

impl Task {
    fn new(
        title: String, 
        description: Option<String>, 
        priority: TaskPriority, 
        category: String
    ) -> Task {
        Task { 
            id: random::get_string(5, true, false, true, true), 
            title,
            description: {
                match description {
                    Some(value) => Some(value),
                    _ => Some("".to_string())
                }
            }, 
            priority, 
            status: TaskStatus::Pending, 
            category,
        }
    }
}