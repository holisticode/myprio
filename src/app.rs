use std::fmt::Display;
use std::str::FromStr;

use crate::error::{EnumParseError, Error, Result};
use crate::task::manager::TaskManager;
use crate::task::{Task, TaskPriority, TaskStatus};
use clap::{Args, Parser, Subcommand, ValueEnum};
use home;
use inquire::{Confirm, Editor, Select, Text};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

//const DESC_LEN: u8 = 32;
const TASKS_DB_FILE_NAME: &str = "mytasks.sql";

#[derive(Args, Clone)]
pub struct SqlLiteSettings {
    pub datasource: Datasources,
    pub path: String,
}

pub fn default_sqllite_settings() -> SqlLiteSettings {
    let home_dir = match home::home_dir() {
        Some(path) => path,
        None => {
            panic!(
                "I am not able to find out what your home directory is. Use the --path flag to set"
            );
        }
    };
    SqlLiteSettings {
        path: home_dir
            .join(TASKS_DB_FILE_NAME)
            .into_os_string()
            .into_string()
            .unwrap(),
        datasource: Datasources::SqlLite,
    }
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(version, about = "cli task management", long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub command: Command,
    #[arg(value_enum)]
    #[clap(group = "ds")]
    pub datasource: Option<Datasources>,
    #[arg(short, long)]
    pub filter: bool,
}

impl App {
    pub fn run_prompt(&self, manager: &mut TaskManager) {
        match &self.command {
            Command::List => {
                let mut filter: Option<(FilterOptions, String)> = None;
                if self.filter {
                    filter = Some(self.run_list_command().unwrap());
                }
                manager.list(&filter);
            }
            Command::Add => match manager.add(self.run_add_command().unwrap()) {
                Ok(_) => log::info!("added task successfully"),
                Err(e) => log::error!("failed to add task: {e:?}"),
            },
            Command::Show => match manager.show(self.ask_task_id().unwrap()) {
                Ok(_) => log::info!("show task run successfully"),
                Err(e) => log::error!("failed to show task: {e:?}"),
            },
            Command::Remove => match manager.remove(self.run_remove_command().unwrap()) {
                Ok(_) => log::info!("removed task successfully"),
                Err(e) => log::error!("failed to remove task: {e:?}"),
            },
            Command::SetStatus => match self.set_status_command(manager) {
                Ok(_) => log::info!("task updated successfully"),
                Err(e) => log::error!("failed to update task: {e:?}"),
            },

            _ => todo!(),
        }
    }

    fn run_list_command(&self) -> Result<(FilterOptions, String)> {
        let mut options: Vec<String> = vec![];
        for f in FilterOptions::iter() {
            options.push(f.to_string())
        }
        match FilterOptions::from_str(&Select::new("Filter:", options).prompt()?)? {
            FilterOptions::ByName => Ok((
                FilterOptions::ByName,
                Text::new("Search by name:").prompt()?,
            )),
            FilterOptions::ByStatus => {
                let mut status_opts: Vec<String> = vec![];
                for st in TaskStatus::iter() {
                    status_opts.push(st.to_string())
                }
                Ok((
                    FilterOptions::ByStatus,
                    Select::new("New Status?", status_opts).prompt()?,
                ))
            }
            FilterOptions::ByGroup => Ok((
                FilterOptions::ByName,
                Text::new("Search by group:").prompt()?,
            )),
            FilterOptions::ByPriority => {
                let mut prio_opts: Vec<String> = vec![];
                for st in TaskPriority::iter() {
                    prio_opts.push(st.to_string())
                }
                Ok((
                    FilterOptions::ByPriority,
                    Select::new("New Status?", prio_opts).prompt()?,
                ))
            }
        }
    }

    fn run_add_command(&self) -> Result<Task> {
        let name = Text::new("Task name?").prompt()?;
        let desc = Editor::new("What needs to be done?").prompt()?;
        let mut options: Vec<String> = vec![];
        for p in TaskPriority::iter() {
            options.push(p.to_string());
        }
        let prio = Select::new("What priority to work on it?", options).prompt()?;
        Ok(Task::new(
            name,
            desc,
            TaskPriority::from_str(prio.as_str())?,
        ))
    }

    fn run_remove_command(&self) -> Result<u64> {
        let uid = self.ask_task_id()?;
        println!("WARNING: The remove command ERASES the task from the database.");
        println!("This can not be undone. You could also just change status or mark the task done");
        let confirm = Confirm::new("Are you really sure you want to delete this task?").prompt()?;
        if confirm {
            return Ok(uid);
        }
        Err(Error::UserAbort)
    }

    fn ask_task_id(&self) -> Result<u64> {
        let strid = Text::new("Task id?").prompt()?;
        let uid = strid.parse::<u64>()?;
        Ok(uid)
    }

    fn set_status_command(&self, manager: &mut TaskManager) -> Result<()> {
        let uid = self.ask_task_id()?;
        let mut task = manager.get_task(uid)?;
        let current_status = task.status.to_string();
        println!("Current status is: {}", current_status);

        let mut options: Vec<String> = vec![];
        for st in TaskStatus::iter() {
            if st.to_string() != current_status {
                options.push(st.to_string())
            }
        }
        let new_status = inquire::Select::new("New Status?", options).prompt()?;
        let ask = format!("Set new status to {}?", new_status);
        let confirm = Confirm::new(&ask).prompt()?;
        if confirm {
            let st = match TaskStatus::from_str(&new_status) {
                Ok(st) => st,
                Err(_) => return Err(crate::error::Error::ParsingError),
            };
            log::debug!("new_status: {}", st.to_string());
            task.status = st;
            manager.set_status(uid, task)?;
        }
        Ok(())
    }
}

#[derive(Subcommand)]
pub enum Command {
    Add,
    Remove,
    Edit,
    List,
    SetStatus,
    Show,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Datasources {
    SqlLite,
}

pub struct NoFilter {}

#[derive(Debug, EnumIter, Eq, PartialEq)]
pub enum FilterOptions {
    ByName,
    ByStatus,
    ByGroup,
    ByPriority,
}

impl Display for FilterOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ByName => write!(f, "By Name"),
            Self::ByStatus => write!(f, "By Status"),
            Self::ByGroup => write!(f, "By Group"),
            Self::ByPriority => write!(f, "By Priority"),
        }
    }
}

impl FromStr for FilterOptions {
    type Err = EnumParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let opt = match s {
            "By Name" => Self::ByName,
            "By Status" => Self::ByStatus,
            "By Group" => Self::ByGroup,
            "By Priority" => Self::ByPriority,
            _ => return Err(EnumParseError {}),
        };
        Ok(opt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use home;

    #[test]
    fn default_settings() {
        let settings = default_sqllite_settings();
        assert_eq!(settings.datasource, Datasources::SqlLite);
        let db = home::home_dir().unwrap().join(TASKS_DB_FILE_NAME);
        assert_eq!(settings.path, db.to_string_lossy());
    }
}
