use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct InvalidCommandError;

#[derive(Debug, Clone)]
pub struct ErrorStatusError {
    description: String
}

impl ErrorStatusError {
    pub fn new(description: &str) -> ErrorStatusError {
        ErrorStatusError { description: description.to_string() }
    }
}

impl Error for InvalidCommandError {}

impl Error for ErrorStatusError {}

impl fmt::Display for InvalidCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid command")
    }
}

impl fmt::Display for ErrorStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error status {}", self.description)
    }
}
