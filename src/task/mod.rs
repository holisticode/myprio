pub mod manager;

use chrono::{DateTime, Utc};

pub enum TaskStatus {
    Created,
    Started,
    Completed,
    WontDo,
    Archived,
    Deleted,
}

pub struct Task {
    pub id: Option<u64>,
    pub short: String,
    pub desc: String,
    pub created: DateTime<Utc>,
    pub started: Option<DateTime<Utc>>,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(short: String, desc: String) -> Self {
        Self {
            short,
            desc,
            created: Utc::now(),
            id: None,
            started: None,
            status: TaskStatus::Created,
        }
    }
}
