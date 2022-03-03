//! Defines base types such as [`Number`] and [`Identifier`].

use std::str::FromStr;

#[derive(Debug, PartialEq)]
/// Floating-point number container.
pub struct Number {
    value: f64,
}

impl TryFrom<&str> for Number {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            value: f64::from_str(value)?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Contains a alpha-numberic identifier within a [`String`].
pub struct Identifier {
    value: String,
}

impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Self {
            value: String::from(s),
        } 
    }
}
