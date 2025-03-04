use std::num::ParseIntError;

use inquire;
use rusqlite;

use crate::task::NoSuchStatusError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TaskNotFound,
    IOError(String),
    InquireError,
    ParsingError,
    UserAbort,
}

#[derive(Debug)]
pub struct EnumParseError;

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Self::IOError(err.to_string())
    }
}

impl From<NoSuchStatusError> for Error {
    fn from(err: NoSuchStatusError) -> Self {
        Self::IOError(err.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        log::error!("Failed to parse data, check format: {}", err);
        Self::ParsingError
    }
}

impl From<inquire::InquireError> for Error {
    fn from(err: inquire::InquireError) -> Self {
        log::error!(
            "Prompting for information generated an unlikely error: {}.",
            err
        );
        Self::InquireError
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err.to_string())
    }
}

impl From<EnumParseError> for Error {
    fn from(_err: EnumParseError) -> Self {
        log::error!("Selected option does not exist");
        Self::ParsingError
    }
}
