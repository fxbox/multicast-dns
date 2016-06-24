use std::error::Error as StdError;
use std::fmt;

use adapters::avahi::errors::Error as AvahiError;

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
    fn description(&self) -> &str {
        match *self {
            Error::AdapterFailure(ref message) => message,
            Error::Internal(ref message) => message,
        }
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}


impl From<AvahiError> for Error {
    fn from(err: AvahiError) -> Error {
        Error::AdapterFailure(format!("Avahi - {}", err.description()))
    }
}
