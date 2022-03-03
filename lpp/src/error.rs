//! Error types.

use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Error {
    why: String,
}

impl Error {
    pub fn new_boxed<S>(why: S) -> Box<Self>
        where S: Into<String> {
        Box::new(Self {
            why: why.into(),
        })
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lpp error: {}", self.why)
    }
}
