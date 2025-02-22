use std::num::ParseIntError;

use crate::task::manager::TaskManager;
use crate::task::Task;
use clap::{Args, Parser, Subcommand, ValueEnum};
use home;
use inquire::{Confirm, Editor, Text};

const DESC_LEN: u8 = 32;

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
            .join("mytasks.sql")
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
            Command::Add => match manager.add(self.run_add_command()) {
                Ok(_) => log::info!("added task successfully"),
                Err(e) => log::error!("failed to add task: {e:?}"),
            },
            Command::List => manager.list(),
            Command::Remove => match manager.remove(self.run_remove_command()) {
                Ok(_) => log::info!("removed task successfully"),
                Err(e) => log::error!("failed to remove task: {e:?}"),
            },

            _ => todo!(),
        }
    }

    fn run_add_command(&self) -> Task {
        let name = Text::new("Task name?").prompt();
        let strname = match name {
            Ok(name) => {
                println!("Task name: {}", name);
                name
            }
            Err(_) => {
                println!("An error happened when asking for your name, try again later.");
                String::from("error")
            }
        };
        let desc = Editor::new("What needs to be done?").prompt();
        let strdesc = match desc {
            Ok(desc) => {
                let mut desc_len: usize = DESC_LEN as usize;
                if desc.len() < DESC_LEN as usize {
                    desc_len = desc.len();
                }
                println!("Task description: {}", &desc[..desc_len]);
                desc
            }
            Err(_) => {
                println!("An error happened when asking for your name, try again later.");
                String::from("error")
            }
        };
        Task::new(strname, strdesc)
    }

    fn run_remove_command(&self) -> u64 {
        let id = Text::new("Task id?").prompt();
        println!("WARNING: The remove command ERASES the task from the database.");
        println!("This can not be undone. You could also just change status or mark the task done");
        let confirm =
            match Confirm::new("Are you really sure you want to delete this task?").prompt() {
                Ok(b) => b,
                Err(err) => {
                    println!("An error occurred asking for confirmation, try again later");
                    false
                }
            };
        if confirm {
            let strid = id.unwrap();
            let uid: u64 = match strid.parse() {
                Ok(uid) => uid,
                Err(_) => 0,
            };
            return uid;
        }
        0
    }
}

#[derive(Subcommand)]
pub enum Command {
    Add,
    Remove,
    Edit,
    List,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Datasources {
    SqlLite,
    Memory,
}
