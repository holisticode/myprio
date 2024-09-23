use crate::task::manager::TaskManager;
use crate::task::Task;
use clap::{Parser, Subcommand, ValueEnum};
use inquire::{Editor, Text};

const DESC_LEN: u8 = 32;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(version, about = "cli task management", long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub command: Command,
    #[arg(value_enum)]
    pub datasource: Datasources,
    pub path: Option<String>,
}

impl App {
    pub fn run_prompt(&self, manager: &mut TaskManager) {
        match &self.command {
            Command::Add => match manager.add(self.run_add_command()) {
                Ok(_) => log::info!("added task successfully"),
                Err(e) => log::error!("failed to add task: {e:?}"),
            },
            Command::List => manager.list(),

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
