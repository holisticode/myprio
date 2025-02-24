pub mod manager;

use std::str::FromStr;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

use chrono::{DateTime, Utc};
use strum_macros::EnumIter;

pub struct NoSuchStatusError {}
impl Error for NoSuchStatusError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for NoSuchStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Status does not exist")
    }
}

impl Debug for NoSuchStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoSuchStatusError").finish()
    }
}

#[derive(Debug, EnumIter)]
pub enum TaskStatus {
    Created,
    Started,
    Completed,
    WontDo,
    Archived,
    Deleted,
}

pub type Timestamp = DateTime<Utc>;

#[derive(Debug)]
pub struct Task {
    pub id: Option<u64>,
    pub short: String,
    pub desc: String,
    pub created: Timestamp,
    pub started: Option<Timestamp>,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(short: String, desc: String) -> Self {
        Self {
            short,
            desc,
            created: Utc::now(),
            id: None,
            started: None,
            status: TaskStatus::Created,
        }
    }
}

// TODO
#[allow(clippy::to_string_trait_impl)]
impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Created => "Created".to_string(),
            Self::Started => "Started".to_string(),
            Self::Completed => "Completed".to_string(),
            Self::WontDo => "WontDo".to_string(),
            Self::Archived => "Archived".to_string(),
            Self::Deleted => "Deleted".to_string(),
        }
    }
}

impl FromStr for TaskStatus {
    type Err = NoSuchStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Created" => Ok(Self::Created),
            "Started" => Ok(Self::Started),
            "Completed" => Ok(Self::Completed),
            "WontDo" => Ok(Self::WontDo),
            "Archived" => Ok(Self::Archived),
            "Deleted" => Ok(Self::Deleted),
            _ => Err(NoSuchStatusError {}),
        }
    }
}
