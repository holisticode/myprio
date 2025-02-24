pub mod memory;
pub mod sqllite;

use crate::error::Result;
use crate::task::Task;

pub trait Datasource {
    fn start(&self) -> Result<()>;
    fn stop(&self) -> Result<bool>;
    fn remove(&mut self, id: u64) -> Result<usize>;
    fn load_task(&self) -> Result<Task>;
    fn write_task(&mut self, task: Task) -> Result<usize>;
    fn list(&self, tasks: &mut Vec<Task>) -> Result<()>;
    fn get(&self, id: u64) -> Result<Task>;
    fn update_task(&mut self, id: u64, task: Task) -> Result<usize>;
}
