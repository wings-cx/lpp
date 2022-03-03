//! The Lua++ abstract syntax tree.

use std::{str::FromStr, error::Error, fmt::Display};

type AstResult<T> = Result<T, Box<dyn Error>>;

/// Top-level node encompassing the entire program.
#[derive(Debug)]
pub struct Chunk {
    pub identifiers: Vec<Identifier>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            identifiers: Vec::new(),
        }
    }
}

/// Stores an identifier within a [`String`].
#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub value: String,
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self {
            value: String::from(value),
        }
    }
}

macro_rules! identifier_vec {
    ($($item:expr),*) => {
        {
            let mut vec = Vec::new();
            $(vec.push(Identifier::from($item));)*
            vec
        }
    };
}

/// Stores a floating-point number within an [`f64`].
pub struct Number {
    pub value: f64,
}

impl TryFrom<&str> for Number {
    type Error = Box<dyn Error>;
    fn try_from(s: &str) -> AstResult<Number> {
        Ok(Self {
            value: f64::from_str(s)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::ChunkParser;

    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
    fn test_parse() -> TestResult {
        let parser = ChunkParser::new();

        let result = parser.parse("ident1, ident2, _ident3, __ident_4")?;
        assert_eq!(result.identifiers, identifier_vec!["ident1", "ident2", "_ident3", "__ident_4"]);

        Ok(())
    }
}
