use zbus::{fdo::Error as ZBusFdoError, Error as ZBusError};

pub type DBusConsoleResult<T> = Result<T, DBusConsoleError>;

#[derive(Clone, Debug, PartialEq)]
pub enum DBusConsoleError {
    DBusError(String),
    FdoError(String),
}

impl From<ZBusError> for DBusConsoleError {
    fn from(error: ZBusError) -> Self {
        DBusConsoleError::DBusError(error.to_string())
    }
}

impl From<ZBusFdoError> for DBusConsoleError {
    fn from(error: ZBusFdoError) -> Self {
        DBusConsoleError::FdoError(error.to_string())
    }
}
