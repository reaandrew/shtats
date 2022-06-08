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

const ERR_EXECUTING_GIT_VALUE: &str = "Shtats Error: failed to execute git";
const ERR_UNSAFE_GIT_REPOSITORY_VALUE: &str = "Shtats Error: Unsafe repository.  Please visit https://github.blog/2022-04-12-git-security-vulnerability-announced/";
const ERR_NOT_GIT_REPOSITORY: &str = "Shtats Error: Not a git repository.";

// Implement std::fmt::Display for AppError
impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorType::ErrExecutingGit => write!(f, "{}", ERR_EXECUTING_GIT_VALUE),
            ErrorType::ErrUnsafeGitRepository => write!(f,"{}", ERR_UNSAFE_GIT_REPOSITORY_VALUE),
            ErrorType::ErrNotGitRepository => {write!(f,"{}", ERR_NOT_GIT_REPOSITORY)}
        }
    }
}

impl From<std::io::Error> for ShtatsError {
    fn from(e: std::io::Error) -> ShtatsError {
        return ShtatsError::Std(e)
    }
}


impl Error for ShtatsError{}

#[cfg(test)]
mod tests{
    use std::io::{Error, ErrorKind};
    use crate::errors::{ERR_EXECUTING_GIT_VALUE, ERR_NOT_GIT_REPOSITORY, ERR_UNSAFE_GIT_REPOSITORY_VALUE, ErrorType, ShtatsError};

    #[test]
    fn test_reg_error_display_for_err_executing_git(){
        let error = ShtatsError::Regular(ErrorType::ErrExecutingGit);
        assert_eq!(error.to_string(), ERR_EXECUTING_GIT_VALUE)
    }


    #[test]
    fn test_reg_error_display_for_err_unsafe_git_repository(){
        let error = ShtatsError::Regular(ErrorType::ErrUnsafeGitRepository);
        assert_eq!(error.to_string(), ERR_UNSAFE_GIT_REPOSITORY_VALUE)
    }

    #[test]
    fn test_reg_error_display_for_err_not_git_repository(){
        let error = ShtatsError::Regular(ErrorType::ErrNotGitRepository);
        assert_eq!(error.to_string(), ERR_NOT_GIT_REPOSITORY)
    }

    #[test]
    fn test_shtats_error_for_io(){
        let custom_error = Error::new(ErrorKind::Other, "oh no!");
        let error = ShtatsError::Std(custom_error);
        assert_eq!(error.to_string(), "oh no!");
    }

    #[test]
    fn test_shtats_error_from_io(){
        let custom_error = Error::new(ErrorKind::Other, "oh no!");
        let error = ShtatsError::from(custom_error);
        assert_eq!(error.to_string(), "oh no!");
    }
}