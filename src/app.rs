use std::str::FromStr;

use crate::error::{Error, Result};
use crate::task::manager::TaskManager;
use crate::task::{NoSuchStatusError, Task, TaskStatus};
use clap::{Args, Parser, Subcommand, ValueEnum};
use home;
use inquire::{Confirm, Editor, Text};

use strum::IntoEnumIterator;

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
    //#[arg(short,long, num_args(0..))]
    //#[clap(group = "ds")]
    //pub path: String,
}

impl App {
    pub fn run_prompt(&self, manager: &mut TaskManager) {
        match &self.command {
            Command::Add => match manager.add(self.run_add_command().unwrap()) {
                Ok(_) => log::info!("added task successfully"),
                Err(e) => log::error!("failed to add task: {e:?}"),
            },
            Command::List => manager.list(),
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

    fn run_add_command(&self) -> Result<Task> {
        let name = Text::new("Task name?").prompt()?;
        let desc = Editor::new("What needs to be done?").prompt()?;
        Ok(Task::new(name, desc))
    }

    fn run_remove_command(&self) -> Result<u64> {
        let strid = Text::new("Task id?").prompt()?;
        println!("WARNING: The remove command ERASES the task from the database.");
        println!("This can not be undone. You could also just change status or mark the task done");
        let confirm = Confirm::new("Are you really sure you want to delete this task?").prompt()?;
        if confirm {
            return Ok(strid.parse::<u64>()?);
        }
        Err(Error::UserAbort)
    }

    fn set_status_command(&self, manager: &mut TaskManager) -> Result<()> {
        let strid = Text::new("Task id?").prompt()?;
        let uid = strid.parse::<u64>()?;
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
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Datasources {
    SqlLite,
    Memory,
}
