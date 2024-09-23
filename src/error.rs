use sqlite;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TaskNotFound,
    IOError(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err.to_string())
    }
}

impl From<sqlite::Error> for Error {
    fn from(err: sqlite::Error) -> Self {
        Self::IOError(err.to_string())
    }
}
