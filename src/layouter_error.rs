use std::fmt;
use std::io::Error;

#[derive(Debug)]
pub struct LayouterError {
    pub description: String,
    pub io_error: Option<Error>,
}

impl fmt::Display for LayouterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl std::error::Error for LayouterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Some(err) = &self.io_error {
            Some(err)
        } else {
            None
        }
    }
}

impl LayouterError {
    pub fn from_description(description: String) -> Self {
        Self {
            description,
            io_error: None,
        }
    }
    pub fn from_ioerror(io_error: Error) -> Self {
        Self {
            description: "IoError".to_string(),
            io_error: Some(io_error),
        }
    }
}

pub type Result<T> = std::result::Result<T, LayouterError>;
