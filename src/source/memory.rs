use crate::error::Result;
use crate::source::Datasource;
use crate::task::Task;

pub struct MemoryDataSource {
    settings: MemorySettings,
    tasks: Vec<Task>,
}

pub struct MemorySettings {
    // maybe max num of tasks?
}

impl MemoryDataSource {
    pub fn new() -> Result<Self> {
        let settings = MemorySettings {};
        Ok(Self {
            settings,
            tasks: vec![],
        })
    }
}

impl Datasource for MemoryDataSource {
    fn start(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn stop(&self) -> crate::error::Result<bool> {
        todo!()
    }

    fn load_task(&self) -> crate::error::Result<Task> {
        todo!()
    }

    fn write_task(&mut self, task: Task) -> crate::error::Result<bool> {
        self.tasks.push(task);
        Ok(true)
    }

    fn list(&self, _: &mut Vec<Task>) -> Result<()> {
        todo!()
    }
}
