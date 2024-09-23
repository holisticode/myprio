use log;
use sqlite::State;
use std::path::Path;

use crate::error::Result;
use crate::source::Datasource;
use crate::task::Task;

pub struct SqlLiteDataSource {
    settings: SqlLiteSettings,
    conn: sqlite::Connection,
}

pub struct SqlLiteSettings {
    path: String,
}

impl SqlLiteDataSource {
    pub fn new(path: &String) -> Result<Self> {
        let settings = SqlLiteSettings { path: path.clone() };
        let conn = sqlite::open(path)?;
        if !Path::new(&path).exists() {
            let table = "CREATE TABLE tasks(short TEXT, desc TEXT);";
            conn.execute(table)?;
        }
        Ok(Self { settings, conn })
    }
}

impl Datasource for SqlLiteDataSource {
    fn start(&self) -> Result<()> {
        log::info!("running with datasource at: {}", self.settings.path);
        Ok(())
    }

    fn stop(&self) -> Result<bool> {
        todo!()
    }

    fn load_task(&self) -> Result<Task> {
        todo!()
    }

    fn write_task(&mut self, task: Task) -> Result<bool> {
        let insert = format!(
            "INSERT INTO tasks (short,desc) VALUES ('{}','{}');",
            task.short, task.desc
        );
        self.conn.execute(insert)?;
        Ok(true)
    }

    fn list(&self, tasks: &mut Vec<Task>) -> Result<()> {
        let select = "SELECT * from tasks";
        let mut statement = self.conn.prepare(select).unwrap();
        //statement.bind((1, 50)).unwrap();

        while let Ok(State::Row) = statement.next() {
            let short = statement.read::<String, _>("short").unwrap();
            let desc = statement.read::<String, _>("desc").unwrap();
            let t = Task::new(short, desc);
            tasks.push(t);
        }
        Ok(())
    }
}
