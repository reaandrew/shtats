use crate::errors;

pub type Result<T> = std::result::Result<T, errors::ShtatsError>;
