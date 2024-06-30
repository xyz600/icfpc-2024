pub mod ast;
pub mod icfpstring;
pub mod tokenizer;

use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    InvalidCharacter(i64),
    InvalidToken,
    CannotFindNextToken,
    CannotConsumeToken,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidCharacter(i64) => write!(f, "Invalid character {}", i64),
            ParseError::InvalidToken => write!(f, "Invalid token"),
            ParseError::CannotFindNextToken => write!(f, "cannot find next token"),
            ParseError::CannotConsumeToken => write!(f, "cannot consume all token"),
        }
    }
}
