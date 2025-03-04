use log;

use crate::app::FilterOptions;
use crate::error::Result;
use crate::{
    app::Datasources,
    source::{sqllite::SqlLiteDataSource, Datasource},
    task::Task,
};

const TASK_SHORT_LEN: usize = 30;
const TASK_DESC_LEN: usize = 40;

pub struct TaskManager {
    source: Box<dyn Datasource>,
}

impl TaskManager {
    pub fn add(&mut self, task: Task) -> Result<usize> {
        self.source.write_task(task)
    }

    // TODO: path should probably be a PathBuf or Path
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

    pub fn list(&mut self, filter: &Option<(FilterOptions, String)>) {
        let mut tasks: Vec<Task> = Vec::new();
        match self.source.list(&mut tasks, filter) {
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

    pub(crate) fn show(&self, id: u64) -> Result<()> {
        let task = self.source.get(id)?;
        println!("==================================================================================================================");
        println!(" Show info for task: {}", task.id.unwrap());
        println!("------------------------------------------------------------------------------------------------------------------");
        println!(" ID: {}", task.id.unwrap());
        println!(" Task: {}", task.short);
        println!(" Description: {}", task.desc);
        println!(" Priority: {}", task.prio.to_string());
        println!(" Status: {}", task.status.to_string());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile;

    use super::*;
    use crate::task::{TaskPriority, TaskStatus};

    #[test]
    fn create_db() -> Result<()> {
        let ds = Datasources::SqlLite;
        let f = tempfile::NamedTempFile::new()?;
        let path = f.path();
        let tmpfile = path.to_string_lossy();

        // because we created the tempfile already, it should actually already exist...
        assert!(fs::exists(path).expect("this is a temp file and should exist"));
        let _mgr = TaskManager::new(ds, &tmpfile.to_string());
        // should still be there
        assert!(fs::exists(path).expect("this file should now exist"));

        // should fail, non-existing path
        let should_panic =
            std::panic::catch_unwind(|| TaskManager::new(ds, &"/root/no_perms.sql".to_string()));
        assert!(should_panic.is_err());
        Ok(())
    }

    #[test]
    fn ops() -> Result<()> {
        let ds = Datasources::SqlLite;
        let f = tempfile::NamedTempFile::new()?;
        let path = f.path();
        let tmpfile = path.to_string_lossy();

        // because we created the tempfile already, it should actually already exist...
        assert!(fs::exists(path).expect("this is a temp file and should exist"));
        let mut mgr = TaskManager::new(ds, &tmpfile.to_string());
        // listing should just work
        mgr.list(&None);

        let t = Task::new(
            "test42".to_string(),
            "test4242".to_string(),
            TaskPriority::UrgentAndImportant,
        );
        // add a new task
        assert_eq!(mgr.add(t).ok(), Some(1));
        assert!(mgr.get_task(99).is_err());
        // get it and compare
        let mut ctrl = mgr.get_task(1);
        assert!(ctrl.is_ok());
        let mut ctrl_task = ctrl.unwrap();
        assert_eq!(ctrl_task.id, Some(1));
        assert_eq!(ctrl_task.short, "test42");
        assert_eq!(ctrl_task.desc, "test4242");
        assert_eq!(ctrl_task.prio, TaskPriority::UrgentAndImportant);
        assert!(matches!(ctrl_task.status, TaskStatus::Created));

        // update status
        let mut update = ctrl_task.clone();
        update.status = TaskStatus::WontDo;
        assert_eq!(mgr.set_status(1, update).ok(), Some(1));
        ctrl = mgr.get_task(1);
        assert!(ctrl.is_ok());
        ctrl_task = ctrl.unwrap();
        assert_eq!(ctrl_task.id, Some(1));
        assert_eq!(ctrl_task.short, "test42");
        assert_eq!(ctrl_task.desc, "test4242");
        assert_eq!(ctrl_task.prio, TaskPriority::UrgentAndImportant);
        assert!(matches!(ctrl_task.status, TaskStatus::WontDo));

        assert_eq!(mgr.remove(1).ok(), Some(1));
        ctrl = mgr.get_task(1);
        assert!(ctrl.is_err());
        Ok(())
    }
}
