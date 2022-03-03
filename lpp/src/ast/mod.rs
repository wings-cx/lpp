//! Contains the abstract syntax tree defining a Lua++ program.

mod statement;
mod expression;
mod primitive;

pub use statement::*;
pub use expression::*;
pub use primitive::*;

/// Top-level node containing a list of zero or more statements, and an optional return statement.
#[derive(Debug)]
pub struct Chunk {
    statements: Vec<Statement>,
    return_statement: Option<ReturnStatement>,
}

impl Chunk {
    pub fn new(statements: Vec<Statement>, return_statement: Option<ReturnStatement>) -> Self {
        Self {
            statements,
            return_statement,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::language::ChunkParser;

    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let parser = ChunkParser::new();
        let ast = parser.parse("local a, b = 10.0, 34.1")?;
        println!("{:?}", ast);

        Ok(())
    }
}
