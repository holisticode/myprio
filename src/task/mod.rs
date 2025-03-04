pub mod manager;

use std::cmp::Ordering;
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

#[derive(Debug, EnumIter, Clone)]
pub enum TaskStatus {
    Created,
    Started,
    Completed,
    WontDo,
    Archived,
    Deleted,
}

#[derive(Debug, PartialEq, Eq, EnumIter, Clone)]
pub enum TaskPriority {
    UrgentAndImportant,
    UrgentNotImportant,
    ImportantNotUrgent,
    ToDo,
    Watch,
    NiceToDo,
    SomeDay,
}

pub type Timestamp = DateTime<Utc>;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Option<u64>,
    pub short: String,
    pub desc: String,
    pub created: Timestamp,
    pub started: Option<Timestamp>,
    pub status: TaskStatus,
    pub prio: TaskPriority,
}

impl Task {
    pub fn new(short: String, desc: String, prio: TaskPriority) -> Self {
        Self {
            short,
            desc,
            created: Utc::now(),
            id: None,
            started: None,
            status: TaskStatus::Created,
            prio: prio,
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

impl TaskPriority {
    fn get_ordering_value(&self) -> u8 {
        match self {
            Self::UrgentAndImportant => 1,
            Self::UrgentNotImportant => 2,
            Self::ImportantNotUrgent => 3,
            Self::ToDo => 4,
            Self::Watch => 5,
            Self::NiceToDo => 6,
            Self::SomeDay => 7,
        }
    }
}

impl PartialOrd for TaskPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_ordering_value()
            .partial_cmp(&other.get_ordering_value())
    }
}

impl Ord for TaskPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_ordering_value().cmp(&other.get_ordering_value())
    }
}

// TODO
#[allow(clippy::to_string_trait_impl)]
impl ToString for TaskPriority {
    fn to_string(&self) -> String {
        match self {
            Self::UrgentAndImportant => "Urgent AND Important".to_string(),
            Self::UrgentNotImportant => "Urgent NOT Important".to_string(),
            Self::ImportantNotUrgent => "Important Not Urgent".to_string(),
            Self::ToDo => "To Do".to_string(),
            Self::Watch => "Watch".to_string(),
            Self::SomeDay => "Some Day".to_string(),
            Self::NiceToDo => "Nice To Do".to_string(),
        }
    }
}

impl FromStr for TaskPriority {
    type Err = NoSuchStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Urgent AND Important" => Ok(Self::UrgentAndImportant),
            "Urgent NOT Important" => Ok(Self::UrgentNotImportant),
            "Important Not Urgent" => Ok(Self::ImportantNotUrgent),
            "To Do" => Ok(Self::ToDo),
            "Watch" => Ok(Self::Watch),
            "Some Day" => Ok(Self::SomeDay),
            "Nice To Do" => Ok(Self::NiceToDo),
            //TODO
            _ => Err(NoSuchStatusError {}),
        }
    }
}
