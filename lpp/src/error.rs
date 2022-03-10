//! Defines a basic [`Error`] and [`Failable`] type.

use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    why: String,
}

impl Error {
    /// Returns a new boxed [`Error`] with the `why` passed.
    pub fn new<S: Into<String>>(why: S) -> Box<Self> {
        Box::new(Self {
            why: why.into(),
        })
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.why)
    }
}

impl std::error::Error for Error {
}

/// Wraps a [`std::result::Result`] with a boxed [`std::error::Error`] as an error type.
pub type Failable<T> = Result<T, Box<dyn std::error::Error>>;
