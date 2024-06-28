pub mod ifcpstring;

use std::fmt::Display;

pub enum TokenType {
    BooleanTrue,
    BooleanFalse,
    Integer(i64),
    String(String),
    UnaryNegate,
    UnaryNot,
    UnaryStrToInt,
    UnaryIntToStr,
    BinaryAdd,
    BinarySub,
    BinaryMul,
    BinaryDiv,
    BinaryModulo,
    BinaryIntegerLarger,
    BinaryIntegerSmaller,
    BinaryEqual,
    BinaryOr,
    BinaryAnd,
    BinaryStrConcat,
    BinaryTakeStr,
    BinaryDropStr,
    BinaryApply,
    If,
    Lambda(i64),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    InvalidCharacter(i64),
    InvalidToken,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidCharacter(i64) => write!(f, "Invalid character {}", i64),
            ParseError::InvalidToken => write!(f, "Invalid token"),
        }
    }
}
