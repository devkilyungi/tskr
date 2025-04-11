use random_str as random;

pub struct Task {
    pub id: i32,
    title: String,
    description: String,
    priority: TaskPriority,
    pub status: TaskStatus,
    pub category: String,
}

pub enum TaskStatus {
    Pending,
    Completed,
}

impl TaskStatus {
    pub fn to_string(&self) -> String {
        match self {
            TaskStatus::Pending => "Pending".to_string(),
            TaskStatus::Completed => "Completed".to_string(),
        }
    }
}

pub enum TaskPriority {
    Low,
    Medium,
    High,
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

    pub fn to_string(&self) -> String {
        match self {
            TaskPriority::Low => "Low".to_string(),
            TaskPriority::Medium => "Medium".to_string(),
            TaskPriority::High => "High".to_string(),
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
            self.priority.to_string(),
            self.category,
            self.status.to_string()
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
}
