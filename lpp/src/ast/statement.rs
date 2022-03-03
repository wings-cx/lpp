//! Defines the [`Statement`] enum which encompasses all statement types.

use crate::{ast::{Identifier, Expression}, error::{Result, Error}};

#[derive(Debug)]
pub enum Statement {
    LocalAssign(LocalAssignStatement),
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub results: Vec<Expression>,
}

#[derive(Debug)]
pub struct LocalAssignStatement {
    pub assignments: Vec<(Identifier, Expression)>,
}

impl LocalAssignStatement {
    pub fn new(identifiers: Vec<Identifier>, expressions: Vec<Expression>) -> Result<Self> {
        if identifiers.len() != expressions.len() {
            return Err(Error::new_boxed("local assignment: uneven number of identifiers to initializers"));
        }

        Ok(Self {
            assignments: identifiers.into_iter().zip(expressions).collect(),
        })
    }
}
