use std::cmp::Ordering;

use log;
use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    Connection, Row, ToSql,
};

use crate::error::Result;
use crate::source::Datasource;
use crate::task::{Task, TaskPriority, TaskStatus};

pub struct SqlLiteDataSource {
    settings: SqlLiteSettings,
    conn: rusqlite::Connection,
}

pub struct SqlLiteSettings {
    path: String,
}

impl SqlLiteDataSource {
    pub fn new(path: &String) -> Result<Self> {
        let settings = SqlLiteSettings { path: path.clone() };
        log::debug!("SqlLite database path: {}", path);
        let conn = Connection::open(path)?;
        log::debug!("creating new database!");
        let table = "CREATE TABLE IF NOT EXISTS tasks(short TEXT, desc TEXT, status TEXT, created TEXT, started TEXT, prio TEXT);";
        conn.execute(table, ())?;
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

    fn write_task(&mut self, task: Task) -> Result<usize> {
        Ok(self.conn.execute(
            "INSERT INTO tasks (short,desc,status,created,started,prio) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
            (
                task.short,
                task.desc,
                task.status,
                task.created,
                task.started,
                task.prio,
            ),
        )?)
    }

    fn remove(&mut self, id: u64) -> Result<usize> {
        Ok(self
            .conn
            .execute("DELETE from tasks where rowid=?1", [id])?)
    }

    fn list(&self, tasks: &mut Vec<Task>) -> Result<()> {
        let select = "SELECT rowid, * from tasks";
        let mut statement = self.conn.prepare(select).unwrap();

        let tasks_iter = statement.query_map([], |row| row_to_task(row))?;
        for t in tasks_iter {
            tasks.push(t?)
        }
        Ok(())
    }

    fn get(&self, id: u64) -> Result<Task> {
        Ok(self
            .conn
            .query_row("SELECT rowid,* from tasks where rowid=?1", [id], |row| {
                row_to_task(row)
            })?)
    }

    fn update_task(&mut self, id: u64, task: Task) -> Result<usize> {
        Ok(self.conn.execute(
            "UPDATE tasks SET short=?1,desc=?2,status=?3,created=?4,started=?5 where rowid=?6;",
            (
                task.short,
                task.desc,
                task.status,
                task.created,
                task.started,
                id,
            ),
        )?)
    }
}

impl ToSql for TaskPriority {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for TaskPriority {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value
            .as_str()?
            .parse()
            .map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl ToSql for TaskStatus {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for TaskStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value
            .as_str()?
            .parse()
            .map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

fn row_to_task(row: &Row<'_>) -> std::result::Result<Task, rusqlite::Error> {
    Ok(Task {
        id: Some(row.get(0)?),
        short: row.get(1)?,
        desc: row.get(2)?,
        status: row.get(3)?,
        created: row.get(4)?,
        started: row.get(5)?,
        prio: row.get(6)?,
    })
}
