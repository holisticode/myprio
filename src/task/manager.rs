use crate::error::Result;
use log;

use crate::{
    app::Datasources,
    source::{memory::MemoryDataSource, sqllite::SqlLiteDataSource, Datasource},
    task::Task,
};

pub struct TaskManager {
    source: Box<dyn Datasource>,
}

impl TaskManager {
    pub fn add(&mut self, task: Task) -> Result<bool> {
        self.source.write_task(task)
    }

    pub fn list(&mut self) {
        let mut tasks: Vec<Task> = Vec::new();
        match self.source.list(&mut tasks) {
            Ok(tsks) => tsks,
            Err(e) => {
                log::error!("failed to fetch task list from datasource: {e:?}");
                return;
            }
        };
        self.print_task_list(tasks);
    }

    pub fn new(datasource: &Datasources, path: &String) -> Self {
        let ds: Box<dyn Datasource> = match datasource {
            Datasources::SqlLite => Box::new(SqlLiteDataSource::new(path).unwrap_or_else(|e| {
                panic!("creating requested sqllite datasource failed: {e:?}");
            })),
            _ => Box::new(MemoryDataSource::new().unwrap_or_else(|e| {
                panic!("creating requested memory datasource failed: {e:?}");
            })),
        };
        match ds.start() {
            Ok(()) => log::info!("datasource ready"),
            Err(e) => {
                log::error!("datasource failed to initialize: {e:?}");
                panic!("can't proceed without datasource");
            }
        }
        Self { source: ds }
    }

    fn print_task_list(&self, tasks: Vec<Task>) {
        println!("==============================================================================================================");
        println!("Current task list");
        println!("==============================================================================================================");
        println!("|   task name    |        description                                                                        |");
        for t in tasks {
            println!("| {}          | {} ", t.short, t.desc);
        }
        println!("==============================================================================================================");
    }
}
