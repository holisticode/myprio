use crate::error::Result;
use crate::source::Datasource;
use crate::task::Task;

pub struct MemoryDataSource {
    //settings: MemorySettings,
    tasks: Vec<Task>,
}

pub struct MemorySettings {
    // maybe max num of tasks?
}

impl MemoryDataSource {
    pub fn new() -> Result<Self> {
        Ok(Self {
            //settings: MemorySettings {},
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

    fn remove(&mut self, _id: u64) -> Result<usize> {
        todo!()
    }

    fn write_task(&mut self, task: Task) -> crate::error::Result<usize> {
        self.tasks.push(task);
        Ok(1)
    }

    fn list(&self, _: &mut Vec<Task>) -> Result<()> {
        todo!()
    }

    fn get(&self, _id: u64) -> Result<Task> {
        todo!()
    }

    fn update_task(&mut self, _id: u64, _task: Task) -> Result<usize> {
        todo!()
    }
}
