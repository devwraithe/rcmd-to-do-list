use chrono::{DateTime, Utc};

pub struct Task {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed: bool,
}

impl Task {
    pub fn new(
        title: String,
        description: Option<String>,
        due_date: Option<DateTime<Utc>>,
    ) -> Task {
        Task {
            title,
            description,
            due_date,
            completed: false,
        }
    }
}
