//! The Lua++ abstract syntax tree.

use std::{error::Error, str::FromStr};

type AstResult<T> = Result<T, Box<dyn Error>>;

/// Top-level node encompassing the entire program.
#[derive(Debug)]
pub struct Chunk {
    pub statements: Vec<Statement>,
}

impl Chunk {
    /// Returns an empty [`Chunk`].
    pub fn new(statements: Vec<Statement>) -> Self {
        Self {
            statements,
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    LocalAssign(LocalAssignStatement),
}

#[derive(Debug)]
pub struct LocalAssignStatement {
    assignments: Vec<(Identifier, Expression)>,
}

impl LocalAssignStatement {
    pub fn new(identifiers: Vec<Identifier>, values: Vec<Box<Expression>>) -> AstResult<Self> {
        Ok(Self {
            assignments:
        })
    }
}

#[derive(Debug)]
pub enum Expression {
    Number(Number),
    Identifier(Identifier),
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
#[derive(Debug)]
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
        println!("{:?}", parser.parse("a = 10"));

        Ok(())
    }
}
