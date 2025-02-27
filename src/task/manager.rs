use log;

use crate::error::Result;
use crate::{
    app::Datasources,
    source::{sqllite::SqlLiteDataSource, Datasource},
    task::Task,
};

use super::TaskStatus;

const TASK_SHORT_LEN: usize = 30;
const TASK_DESC_LEN: usize = 40;

pub struct TaskManager {
    source: Box<dyn Datasource>,
}

impl TaskManager {
    pub fn add(&mut self, task: Task) -> Result<usize> {
        self.source.write_task(task)
    }

    pub fn new(datasource: Datasources, path: &String) -> Self {
        let ds: Box<dyn Datasource> = match datasource {
            Datasources::SqlLite => Box::new(SqlLiteDataSource::new(path).unwrap_or_else(|e| {
                panic!("creating requested sqllite datasource failed: {e:?}");
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

    pub fn list(&mut self) {
        let mut tasks: Vec<Task> = Vec::new();
        match self.source.list(&mut tasks) {
            Ok(tsks) => tsks,
            Err(e) => {
                log::error!("failed to fetch task list from datasource: {e:?}");
                return;
            }
        };
        self.sort(&mut tasks);
        self.print_task_list(&tasks);
    }

    pub fn remove(&mut self, id: u64) -> Result<usize> {
        self.source.remove(id)
    }

    pub fn set_status(&mut self, id: u64, task: Task) -> Result<usize> {
        log::debug!("{:x?}", task);
        self.source.update_task(id, task)
    }

    pub fn get_task(&mut self, id: u64) -> Result<Task> {
        self.source.get(id)
    }

    fn sort(&self, list: &mut Vec<Task>) {
        list.sort_by(|a, b| a.prio.cmp(&b.prio))
    }

    fn print_task_list(&self, tasks: &Vec<Task>) {
        println!("==================================================================================================================");
        println!("Current task list");
        println!("------------------------------------------------------------------------------------------------------------------");
        println!("| id\t  | task{} | priority{} | status{} | description                                                             |", " ".repeat(TASK_SHORT_LEN - String::from("task").len()), " ".repeat(25 - String::from("priority").len()), " ".repeat(20-String::from("status").len()));
        println!("------------------------------------------------------------------------------------------------------------------");
        for t in tasks {
            let desc_display = t.desc.clone();
            let parts: Vec<&str> = desc_display.split('\n').collect();
            let show = parts[0];
            if show.len() > TASK_DESC_LEN {
                show.to_string().truncate(TASK_DESC_LEN)
            };
            println!(
                "|  {:x?}\t  | {:.30}{} | {:.30}{} | {:.20}{} | {} ",
                t.id.unwrap(),
                t.short,
                " ".repeat(30 - t.short.len()),
                t.prio.to_string(),
                " ".repeat(25 - t.prio.to_string().len()),
                t.status.to_string(),
                " ".repeat(20 - t.status.to_string().len()),
                show,
            );
        }
    }
}
