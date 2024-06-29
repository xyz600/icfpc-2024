use std::collections::VecDeque;

use super::{icfpstring::ICFPString, tokenizer::TokenType, ParseError};

#[derive(Clone, Debug)]
pub enum Node {
    Boolean(bool),
    Integer(i64),
    String(ICFPString),
    UnaryNegate(Box<Node>),
    UnaryNot(Box<Node>),
    UnaryStrToInt(Box<Node>),
    UnaryIntToStr(Box<Node>),
    BinaryAdd(Box<Node>, Box<Node>),
    BinarySub(Box<Node>, Box<Node>),
    BinaryMul(Box<Node>, Box<Node>),
    BinaryDiv(Box<Node>, Box<Node>),
    BinaryModulo(Box<Node>, Box<Node>),
    BinaryIntegerLarger(Box<Node>, Box<Node>),
    BinaryIntegerSmaller(Box<Node>, Box<Node>),
    BinaryEqual(Box<Node>, Box<Node>),
    BinaryOr(Box<Node>, Box<Node>),
    BinaryAnd(Box<Node>, Box<Node>),
    BinaryStrConcat(Box<Node>, Box<Node>),
    BinaryTakeStr(Box<Node>, Box<Node>),
    BinaryDropStr(Box<Node>, Box<Node>),
    BinaryApply(Box<Node>, Box<Node>),
    If(Box<Node>, Box<Node>, Box<Node>),
    Lambda(i64, Box<Node>),
    Variable(i64),
}

pub fn parse(token_stream: &mut VecDeque<TokenType>) -> Result<Node, ParseError> {
    if let Some(token) = token_stream.pop_front() {
        let node = match &token {
            TokenType::Boolean(b) => Node::Boolean(*b),
            TokenType::Integer(i) => Node::Integer(*i),
            TokenType::String(s) => Node::String(s.clone()),
            TokenType::UnaryNegate => {
                let operand = parse(token_stream)?;
                Node::UnaryNegate(Box::new(operand))
            }
            TokenType::UnaryNot => {
                let operand = parse(token_stream)?;
                Node::UnaryNot(Box::new(operand))
            }
            TokenType::UnaryStrToInt => {
                let operand = parse(token_stream)?;
                Node::UnaryStrToInt(Box::new(operand))
            }
            TokenType::UnaryIntToStr => {
                let operand = parse(token_stream)?;
                Node::UnaryIntToStr(Box::new(operand))
            }
            TokenType::BinaryAdd => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::BinaryAdd(Box::new(operand1), Box::new(operand2))
            }
            TokenType::BinarySub => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::BinarySub(Box::new(operand1), Box::new(operand2))
            }
            TokenType::BinaryMul => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::BinaryMul(Box::new(operand1), Box::new(operand2))
            }
            TokenType::BinaryDiv => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::BinaryDiv(Box::new(operand1), Box::new(operand2))
            }
            TokenType::BinaryModulo => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::BinaryModulo(Box::new(operand1), Box::new(operand2))
            }
            TokenType::BinaryIntegerLarger => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::BinaryIntegerLarger(Box::new(operand1), Box::new(operand2))
            }
            TokenType::BinaryIntegerSmaller => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::BinaryIntegerSmaller(Box::new(operand1), Box::new(operand2))
            }
            TokenType::BinaryEqual => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::BinaryEqual(Box::new(operand1), Box::new(operand2))
            }
            TokenType::BinaryOr => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::BinaryOr(Box::new(operand1), Box::new(operand2))
            }
            TokenType::BinaryAnd => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::BinaryAnd(Box::new(operand1), Box::new(operand2))
            }
            TokenType::BinaryStrConcat => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::BinaryStrConcat(Box::new(operand1), Box::new(operand2))
            }
            TokenType::BinaryTakeStr => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::BinaryTakeStr(Box::new(operand1), Box::new(operand2))
            }
            TokenType::BinaryDropStr => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::BinaryDropStr(Box::new(operand1), Box::new(operand2))
            }
            TokenType::BinaryApply => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::BinaryApply(Box::new(operand1), Box::new(operand2))
            }
            TokenType::If => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                let operand3 = parse(token_stream)?;
                Node::If(Box::new(operand1), Box::new(operand2), Box::new(operand3))
            }
            TokenType::Lambda(i) => {
                let operand = parse(token_stream)?;
                Node::Lambda(*i, Box::new(operand))
            }
            TokenType::Variable(i) => Node::Variable(*i),
        };

        // 残りのトークンが消費できなかった場合は、文法エラーとして扱う
        if !token_stream.is_empty() {
            Err(ParseError::CannotConsumeToken)
        } else {
            Ok(node)
        }
    } else {
        Err(ParseError::CannotFindNextToken)
    }
}

pub fn evaluate(node: Node) -> Result<Node, ParseError> {
    match node {
        // 値の場合はそのまま返す
        Node::Boolean(_) | Node::Integer(_) | Node::String(_) | Node::Variable(_) => Ok(node),
        Node::UnaryNegate(child) => {
            let child = evaluate(*child)?;
            match child {
                Node::Integer(i) => Ok(Node::Integer(-i)),
                _ => Err(ParseError::InvalidUnaryNegateOperand),
            }
        }
        Node::UnaryNot(child) => {
            let child = evaluate(*child)?;
            match child {
                Node::Boolean(b) => Ok(Node::Boolean(!b)),
                _ => Err(ParseError::InvalidUnaryNotOperand),
            }
        }
        Node::UnaryStrToInt(child) => {
            let child = evaluate(*child)?;
            match child {
                Node::String(s) => Ok(Node::Integer(s.to_i64())),
                _ => Err(ParseError::InvalidUnaryStrToIntOperand),
            }
        }
        Node::UnaryIntToStr(child) => {
            let child = evaluate(*child)?;
            match child {
                Node::Integer(i) => Ok(Node::String(ICFPString::from_i64(i))),
                _ => Err(ParseError::InvalidUnaryIntToStrOperand),
            }
        }
        Node::BinaryAdd(child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;
            match (child1, child2) {
                (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Integer(i1 + i2)),
                _ => Err(ParseError::InvalidBinaryAddOperand),
            }
        }
        Node::BinarySub(child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;
            match (child1, child2) {
                (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Integer(i1 - i2)),
                _ => Err(ParseError::InvalidBinarySubOperand),
            }
        }
        Node::BinaryMul(child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;
            match (child1, child2) {
                (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Integer(i1 * i2)),
                _ => Err(ParseError::InvalidBinaryMulOperand),
            }
        }
        Node::BinaryDiv(child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;
            // FIXME: check truncated towards zero
            match (child1, child2) {
                (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Integer(i1 / i2)),
                _ => Err(ParseError::InvalidBinaryDivOperand),
            }
        }
        Node::BinaryModulo(child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;
            match (child1, child2) {
                (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Integer(i1 % i2)),
                _ => Err(ParseError::InvalidBinaryModOperand),
            }
        }
        Node::BinaryIntegerLarger(child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;
            match (child1, child2) {
                (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Boolean(i1 > i2)),
                _ => Err(ParseError::InvalidBinaryLtOperand),
            }
        }
        Node::BinaryIntegerSmaller(child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;
            match (child1, child2) {
                (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Boolean(i1 < i2)),
                _ => Err(ParseError::InvalidBinaryGtOperand),
            }
        }
        Node::BinaryEqual(child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;
            match (child1, child2) {
                (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Boolean(i1 == i2)),
                (Node::String(s1), Node::String(s2)) => Ok(Node::Boolean(s1 == s2)),
                (Node::Boolean(b1), Node::Boolean(b2)) => Ok(Node::Boolean(b1 == b2)),
                _ => Err(ParseError::InvalidBinaryEqOperand),
            }
        }
        Node::BinaryOr(child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;
            match (child1, child2) {
                (Node::Boolean(b1), Node::Boolean(b2)) => Ok(Node::Boolean(b1 || b2)),
                _ => Err(ParseError::InvalidBinaryOrOperand),
            }
        }
        Node::BinaryAnd(child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;
            match (child1, child2) {
                (Node::Boolean(b1), Node::Boolean(b2)) => Ok(Node::Boolean(b1 && b2)),
                _ => Err(ParseError::InvalidBinaryAndOperand),
            }
        }
        Node::BinaryStrConcat(child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;
            match (child1, child2) {
                (Node::String(s1), Node::String(s2)) => Ok(Node::String(s1.concat(&s2))),
                _ => Err(ParseError::InvalidBinaryConcatOperand),
            }
        }
        Node::BinaryTakeStr(child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;
            match (child1, child2) {
                (Node::Integer(i), Node::String(s)) => Ok(Node::String(s.take(i as usize))),
                _ => Err(ParseError::InvalidBinaryTakeOperand),
            }
        }
        Node::BinaryDropStr(child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;
            match (child1, child2) {
                (Node::Integer(i), Node::String(s)) => Ok(Node::String(s.drop(i as usize))),
                _ => Err(ParseError::InvalidBinaryDropOperand),
            }
        }
        Node::BinaryApply(child1, child2) => {
            todo!();
        }
        Node::If(pred, first, second) => {
            let pred = evaluate(*pred)?;
            match pred {
                Node::Boolean(b) => {
                    if b {
                        evaluate(*first)
                    } else {
                        evaluate(*second)
                    }
                }
                _ => Err(ParseError::InvalidBinaryIfOperand),
            }
        }
        Node::Lambda(i, child) => {
            todo!();
        }
    }
}
