use std::{error::Error, fmt};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ShtatsError{
    Std(std::io::Error),
    Regular(ErrorType)
}

impl fmt::Display for ShtatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error has occurred")
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum ErrorType {
    ErrExecutingGit,
}

// Implement std::fmt::Display for AppError
impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorType::ErrExecutingGit => write!(f, "Error executing git"),
        }
    }
}

impl From<std::io::Error> for ShtatsError {
    fn from(e: std::io::Error) -> ShtatsError {
        return ShtatsError::Std(e)
    }
}

impl Error for ShtatsError{}