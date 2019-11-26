use json;
use std::error;
use std::fmt;
use std::io;
use std::io::ErrorKind::InvalidData;
use std::num::{ParseFloatError, ParseIntError};
use std::sync::mpsc::RecvError;

#[derive(Debug)]
pub enum PoloError {
    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
    Type(io::Error),
    Json(json::Error),
    Receive(RecvError),
}

impl PoloError {
    pub fn wrong_data(msg: String) -> PoloError {
        PoloError::Type(io::Error::new(InvalidData, msg))
    }
}

impl fmt::Display for PoloError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PoloError::Type(ref err) => write!(f, "Type error: {}", err),
            PoloError::ParseFloat(ref err) => write!(f, "Parse error: {}", err),
            PoloError::ParseInt(ref err) => write!(f, "Parse error: {}", err),
            PoloError::Json(ref err) => write!(f, "Json error: {}", err),
            PoloError::Receive(ref err) => write!(f, "Receive error: {}", err),
        }
    }
}

impl error::Error for PoloError {
    fn description(&self) -> &str {
        match *self {
            PoloError::Type(ref err) => err.description(),
            PoloError::ParseFloat(ref err) => err.description(),
            PoloError::ParseInt(ref err) => err.description(),
            PoloError::Json(ref err) => err.description(),
            PoloError::Receive(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            PoloError::Type(ref err) => Some(err),
            PoloError::ParseFloat(ref err) => Some(err),
            PoloError::ParseInt(ref err) => Some(err),
            PoloError::Json(ref err) => Some(err),
            PoloError::Receive(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for PoloError {
    fn from(err: io::Error) -> PoloError {
        PoloError::Type(err)
    }
}

impl From<ParseFloatError> for PoloError {
    fn from(err: ParseFloatError) -> PoloError {
        PoloError::ParseFloat(err)
    }
}

impl From<ParseIntError> for PoloError {
    fn from(err: ParseIntError) -> PoloError {
        PoloError::ParseInt(err)
    }
}

impl From<json::Error> for PoloError {
    fn from(err: json::Error) -> PoloError {
        PoloError::Json(err)
    }
}

impl From<RecvError> for PoloError {
    fn from(err: RecvError) -> PoloError {
        PoloError::Receive(err)
    }
}
