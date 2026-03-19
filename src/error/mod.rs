use std::fmt;

#[derive(Debug)]
pub enum SortError {
    EmptyInput,
    InvalidData(String),
}

impl fmt::Display for SortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortError::EmptyInput => write!(f, "Cannot sort an empty list"),
            SortError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl std::error::Error for SortError {}
