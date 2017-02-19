use std::error;
use std::fmt;
use std::num::ParseIntError;

/// The error type for USI command conversions.
#[derive(Debug)]
pub enum Error {
    IllegalSyntax,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IllegalSyntax => write!(f, "illegal USI command syntax"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IllegalSyntax => "illegal USI command syntax",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IllegalSyntax => None,
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        Error::IllegalSyntax
    }
}
