pub mod ast;
pub mod icfpstring;
pub mod tokenizer;

use std::{collections::VecDeque, fmt::Display};

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    InvalidCharacter(i64),
    InvalidToken,
    CannotFindNextToken,
    CannotConsumeToken,
    InvalidUnaryNegateOperand,
    InvalidUnaryNotOperand,
    InvalidUnaryStrToIntOperand,
    InvalidUnaryIntToStrOperand,
    InvalidBinaryAddOperand,
    InvalidBinarySubOperand,
    InvalidBinaryMulOperand,
    InvalidBinaryDivOperand,
    InvalidBinaryModOperand,
    InvalidBinaryEqOperand,
    InvalidBinaryNeOperand,
    InvalidBinaryGtOperand,
    InvalidBinaryLtOperand,
    InvalidBinaryAndOperand,
    InvalidBinaryOrOperand,
    InvalidBinaryConcatOperand,
    InvalidBinaryTakeOperand,
    InvalidBinaryDropOperand,
    InvalidBinaryApplyOperand,
    InvalidBinaryIfOperand,
    InvalidBinaryLambdaOperand,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidCharacter(i64) => write!(f, "Invalid character {}", i64),
            ParseError::InvalidToken => write!(f, "Invalid token"),
            ParseError::CannotFindNextToken => write!(f, "cannot find next token"),
            ParseError::CannotConsumeToken => write!(f, "cannot consume all token"),
            ParseError::InvalidUnaryNegateOperand => write!(f, "Invalid unary negate operand"),
            ParseError::InvalidUnaryNotOperand => write!(f, "Invalid unary not operand"),
            ParseError::InvalidUnaryStrToIntOperand => {
                write!(f, "Invalid unary str to int operand")
            }
            ParseError::InvalidUnaryIntToStrOperand => {
                write!(f, "Invalid unary int to str operand")
            }
            ParseError::InvalidBinaryAddOperand => write!(f, "Invalid binary add operand"),
            ParseError::InvalidBinarySubOperand => write!(f, "Invalid binary sub operand"),
            ParseError::InvalidBinaryMulOperand => write!(f, "Invalid binary mul operand"),
            ParseError::InvalidBinaryDivOperand => write!(f, "Invalid binary div operand"),
            ParseError::InvalidBinaryModOperand => write!(f, "Invalid binary mod operand"),
            ParseError::InvalidBinaryEqOperand => write!(f, "Invalid binary eq operand"),
            ParseError::InvalidBinaryNeOperand => write!(f, "Invalid binary ne operand"),
            ParseError::InvalidBinaryLtOperand => write!(f, "Invalid binary lt operand"),
            ParseError::InvalidBinaryGtOperand => write!(f, "Invalid binary gt operand"),
            ParseError::InvalidBinaryAndOperand => write!(f, "Invalid binary and operand"),
            ParseError::InvalidBinaryOrOperand => write!(f, "Invalid binary or operand"),
            ParseError::InvalidBinaryConcatOperand => write!(f, "Invalid binary concat operand"),
            ParseError::InvalidBinaryTakeOperand => write!(f, "Invalid binary take operand"),
            ParseError::InvalidBinaryDropOperand => write!(f, "Invalid binary drop operand"),
            ParseError::InvalidBinaryApplyOperand => write!(f, "Invalid binary apply operand"),
            ParseError::InvalidBinaryIfOperand => write!(f, "Invalid binary if operand"),
            ParseError::InvalidBinaryLambdaOperand => write!(f, "Invalid binary lambda operand"),
        }
    }
}

pub fn parse(input: String) -> Result<ast::Node, ParseError> {
    let token_list = tokenizer::tokenize(input)?;
    let mut queue = VecDeque::from_iter(token_list);
    let mut id = 0;
    let ast = ast::parse(&mut queue, &mut id)?;
    // eprintln!("{}", ast.to_dot_string());
    let evaluated_ast = ast::evaluate(ast)?;
    Ok(evaluated_ast)
}
