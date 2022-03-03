//! Defines the [`Expression`] enum which is the root for most operations.

use super::primitive::{Number, Identifier};

#[derive(Debug)]
pub enum Expression {
    Binary(BinaryExpression),
    Number(Number),
    Identifier(Identifier)
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub op: BinaryOp,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum BinaryOp {
    Mul,
    Div,
    Add,
    Sub,
}
