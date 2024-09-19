use std::io;

use crate::task::task::Task;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn add(&mut self, short: String, desc: String) -> Result<bool, io::Error> {
        let id = self.tasks.len();
        let task = Task::new(short, desc, id as u64);
        self.tasks.push(task);
        Ok(true)
    }

    pub fn new() -> Self {
        Self { tasks: vec![] }
    }
}
