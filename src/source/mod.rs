pub mod memory;
pub mod sqllite;

use crate::error::Result;
use crate::task::Task;

pub trait Datasource {
    fn start(&self) -> Result<()>;
    fn stop(&self) -> Result<bool>;
    fn load_task(&self) -> Result<Task>;
    fn write_task(&mut self, task: Task) -> Result<bool>;
    fn list(&self, tasks: &mut Vec<Task>) -> Result<()>;
}
