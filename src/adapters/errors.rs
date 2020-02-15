use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    AdapterFailure(String),
    Internal(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::AdapterFailure(ref message) => f.write_str(message),
            Error::Internal(ref message) => f.write_str(message),
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}

#[cfg(target_os = "linux")]
use adapters::avahi;
#[cfg(target_os = "linux")]
impl From<avahi::errors::Error> for Error {
    fn from(err: avahi::errors::Error) -> Error {
        Error::AdapterFailure(format!("Avahi - {}", err))
    }
}
