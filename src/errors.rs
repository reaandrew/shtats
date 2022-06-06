use std::{error::Error, fmt};
use serde::{Deserialize, Serialize};
use crate::errors::ShtatsError::{Regular, Std};

#[derive(Debug)]
pub enum ShtatsError{
    Std(std::io::Error),
    Regular(ErrorType)
}

impl fmt::Display for ShtatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Std(value) => write!(f,"{}", value),
            Regular(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum ErrorType {
    ErrExecutingGit,
    ErrUnsafeGitRepository,
    ErrNotGitRepository
}

// Implement std::fmt::Display for AppError
impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorType::ErrExecutingGit => write!(f, "Shtats Error: failed to execute git"),
            ErrorType::ErrUnsafeGitRepository => write!(f, "Shtats Error: Unsafe repository.  Please visit https://github.blog/2022-04-12-git-security-vulnerability-announced/"),
            ErrorType::ErrNotGitRepository => {write!(f, "Shtats Error: Not a git repository.")}
        }
    }
}

impl From<std::io::Error> for ShtatsError {
    fn from(e: std::io::Error) -> ShtatsError {
        return ShtatsError::Std(e)
    }
}


impl Error for ShtatsError{}