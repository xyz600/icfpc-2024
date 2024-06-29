pub mod ast;
pub mod icfpstring;
pub mod tokenizer;

use std::{collections::VecDeque, fmt::Display};

use ast::NodeFactory;

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

pub fn parse(input: String) -> Result<ast::Node, ParseError> {
    let token_list = tokenizer::tokenize(input)?;
    let mut queue = VecDeque::from_iter(token_list);
    let mut node_factory = NodeFactory::new();
    let ast = ast::parse(&mut queue, &mut node_factory)?;
    eprintln!("{}", ast.to_dot_string());
    let evaluated_ast = ast::evaluate(ast, &mut node_factory)?;
    Ok(evaluated_ast)
}
