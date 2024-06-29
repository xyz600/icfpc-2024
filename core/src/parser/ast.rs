use std::collections::VecDeque;

use super::{
    icfpstring::ICFPString,
    tokenizer::{BinaryOpecode, TokenType, UnaryOpecode},
    ParseError,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Boolean(bool),
    Integer(i64),
    String(ICFPString),
    Unary(UnaryOpecode, Box<Node>),
    Binary(BinaryOpecode, Box<Node>, Box<Node>),
    If(Box<Node>, Box<Node>, Box<Node>),
    Lambda(i64, Box<Node>),
    Variable(i64),
}

impl Node {
    fn visit(&self, f: &mut impl FnMut(&Node)) {
        f(&self);
        match self {
            Node::Boolean(_) | Node::Integer(_) | Node::String(_) | Node::Variable(_) => {}
            Node::Unary(_, child) => child.visit(f),
            Node::Binary(_, child1, child2) => {
                child1.visit(f);
                child2.visit(f);
            }
            Node::If(pred, first, second) => {
                pred.visit(f);
                first.visit(f);
                second.visit(f);
            }
            Node::Lambda(_, child) => child.visit(f),
        }
    }

    fn visit_mut(&mut self, f: &mut impl FnMut(&mut Node)) {
        f(self);
        match self {
            Node::Boolean(_) | Node::Integer(_) | Node::String(_) | Node::Variable(_) => {}
            Node::Unary(_, child) => child.visit_mut(f),
            Node::Binary(_, child1, child2) => {
                child1.visit_mut(f);
                child2.visit_mut(f);
            }
            Node::If(pred, first, second) => {
                pred.visit_mut(f);
                first.visit_mut(f);
                second.visit_mut(f);
            }
            Node::Lambda(_, child) => child.visit_mut(f),
        }
    }
}

pub fn parse(token_stream: &mut VecDeque<TokenType>) -> Result<Node, ParseError> {
    if let Some(token) = token_stream.pop_front() {
        let node = match &token {
            TokenType::Boolean(b) => Node::Boolean(*b),
            TokenType::Integer(i) => Node::Integer(*i),
            TokenType::String(s) => Node::String(s.clone()),
            TokenType::Unary(opcode) => {
                let operand = parse(token_stream)?;
                Node::Unary(*opcode, Box::new(operand))
            }
            TokenType::Binary(opcode) => {
                let operand1 = parse(token_stream)?;
                let operand2 = parse(token_stream)?;
                Node::Binary(*opcode, Box::new(operand1), Box::new(operand2))
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
        Ok(node)
    } else {
        Err(ParseError::CannotFindNextToken)
    }
}

pub fn evaluate(node: Node) -> Result<Node, ParseError> {
    match node {
        // 値の場合はそのまま返す
        Node::Boolean(_) | Node::Integer(_) | Node::String(_) | Node::Variable(_) => Ok(node),
        Node::Unary(opcode, child) => {
            let child = evaluate(*child)?;
            match opcode {
                UnaryOpecode::Negate => match child {
                    Node::Integer(i) => Ok(Node::Integer(-i)),
                    _ => Err(ParseError::InvalidUnaryNegateOperand),
                },
                UnaryOpecode::Not => match child {
                    Node::Boolean(b) => Ok(Node::Boolean(!b)),
                    _ => Err(ParseError::InvalidUnaryNotOperand),
                },
                UnaryOpecode::StrToInt => match child {
                    Node::String(s) => Ok(Node::Integer(s.to_i64())),
                    _ => Err(ParseError::InvalidUnaryStrToIntOperand),
                },
                UnaryOpecode::IntToStr => match child {
                    Node::Integer(i) => Ok(Node::String(ICFPString::from_i64(i))),
                    _ => Err(ParseError::InvalidUnaryIntToStrOperand),
                },
            }
        }
        Node::Binary(opcode, child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;
            match opcode {
                BinaryOpecode::Add => match (child1, child2) {
                    (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Integer(i1 + i2)),
                    _ => Err(ParseError::InvalidBinaryAddOperand),
                },
                BinaryOpecode::Sub => match (child1, child2) {
                    (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Integer(i1 - i2)),
                    _ => Err(ParseError::InvalidBinarySubOperand),
                },
                BinaryOpecode::Mul => match (child1, child2) {
                    (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Integer(i1 * i2)),
                    _ => Err(ParseError::InvalidBinaryMulOperand),
                },
                BinaryOpecode::Div => {
                    // FIXME: check truncated towards zero
                    match (child1, child2) {
                        (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Integer(i1 / i2)),
                        _ => Err(ParseError::InvalidBinaryDivOperand),
                    }
                }
                BinaryOpecode::Modulo => match (child1, child2) {
                    (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Integer(i1 % i2)),
                    _ => Err(ParseError::InvalidBinaryModOperand),
                },
                BinaryOpecode::IntegerLarger => match (child1, child2) {
                    (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Boolean(i1 < i2)),
                    _ => Err(ParseError::InvalidBinaryLtOperand),
                },
                BinaryOpecode::IntegerSmaller => match (child1, child2) {
                    (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Boolean(i1 > i2)),
                    _ => Err(ParseError::InvalidBinaryGtOperand),
                },
                BinaryOpecode::Equal => match (child1, child2) {
                    (Node::Integer(i1), Node::Integer(i2)) => Ok(Node::Boolean(i1 == i2)),
                    (Node::String(s1), Node::String(s2)) => Ok(Node::Boolean(s1 == s2)),
                    (Node::Boolean(b1), Node::Boolean(b2)) => Ok(Node::Boolean(b1 == b2)),
                    _ => Err(ParseError::InvalidBinaryEqOperand),
                },
                BinaryOpecode::Or => match (child1, child2) {
                    (Node::Boolean(b1), Node::Boolean(b2)) => Ok(Node::Boolean(b1 || b2)),
                    _ => Err(ParseError::InvalidBinaryOrOperand),
                },
                BinaryOpecode::And => match (child1, child2) {
                    (Node::Boolean(b1), Node::Boolean(b2)) => Ok(Node::Boolean(b1 && b2)),
                    _ => Err(ParseError::InvalidBinaryAndOperand),
                },
                BinaryOpecode::StrConcat => match (child1, child2) {
                    (Node::String(s1), Node::String(s2)) => Ok(Node::String(s1.concat(&s2))),
                    _ => Err(ParseError::InvalidBinaryConcatOperand),
                },
                BinaryOpecode::TakeStr => match (child1, child2) {
                    (Node::Integer(i), Node::String(s)) => Ok(Node::String(s.take(i as usize))),
                    _ => Err(ParseError::InvalidBinaryTakeOperand),
                },
                BinaryOpecode::DropStr => match (child1, child2) {
                    (Node::Integer(i), Node::String(s)) => Ok(Node::String(s.drop(i as usize))),
                    _ => Err(ParseError::InvalidBinaryDropOperand),
                },
                BinaryOpecode::Apply => {
                    // NOTE: どのような順序で簡約しても影響がないと仮定している
                    // 1. child2 を eval する
                    // 2. child1 が Lambda だったら、 child1 の中の Variable(i) をみつけて、 child2 に置き換える
                    // 3. child1 が Lambda でなかったら、 child1 と child2 を評価をしつつ、そのまま返す

                    match child1 {
                        Node::Lambda(i, child) => {
                            let mut child = *child;
                            child.visit_mut(&mut |node| {
                                if let Node::Variable(j) = node {
                                    if *j == i {
                                        *node = child2.clone();
                                    }
                                }
                            });
                            Ok(child)
                        }
                        _ => Ok(Node::Binary(
                            BinaryOpecode::Apply,
                            Box::new(child1),
                            Box::new(child2),
                        )),
                    }
                }
            }
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
            let child = evaluate(*child)?;
            Ok(Node::Lambda(i, Box::new(child)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tokenizer::tokenize;

    fn test_sequence(input: &str, expected: Node) {
        let token_list = tokenize(input.to_string()).unwrap();
        let mut token_stream = VecDeque::from(token_list);
        let node = parse(&mut token_stream).unwrap();
        eprintln!("{:?}", node);
        let result = evaluate(node).unwrap();
        assert_eq!(result, expected);
    }

    // testcase is generated from https://icfpcontest2024.github.io/icfp.html

    #[test]
    fn test_add() {
        test_sequence("B+ I# I$", Node::Integer(5));
    }

    #[test]
    fn test_sub() {
        test_sequence("B- I$ I#", Node::Integer(1));
    }

    #[test]
    fn test_mul() {
        test_sequence("B* I# I$", Node::Integer(6));
    }

    #[test]
    fn test_div() {
        test_sequence("B/ U- I( I#", Node::Integer(-3));
    }

    #[test]
    fn test_mod() {
        test_sequence("B% U- I( I#", Node::Integer(-1));
    }

    #[test]
    fn test_gt() {
        test_sequence("B< I$ I#", Node::Boolean(false));
        test_sequence("B< I# I$", Node::Boolean(true));
    }

    #[test]
    fn test_lt() {
        test_sequence("B> I$ I#", Node::Boolean(true));
        test_sequence("B> I# I$", Node::Boolean(false));
    }

    #[test]
    fn test_eq() {
        test_sequence("B= I$ I#", Node::Boolean(false));
        test_sequence("B= I$ B+ I# I\"", Node::Boolean(true));

        test_sequence("B= S# S#", Node::Boolean(true));
        test_sequence("B= S# S$", Node::Boolean(false));

        test_sequence("B= T B= F F", Node::Boolean(true));
        test_sequence("B= F B= F F", Node::Boolean(false));
    }

    #[test]
    fn test_and() {
        test_sequence("B& T F", Node::Boolean(false));
        test_sequence("B& T T", Node::Boolean(true));
    }

    #[test]
    fn test_or() {
        test_sequence("B| T F", Node::Boolean(true));
        test_sequence("B| F F", Node::Boolean(false));
    }

    #[test]
    fn test_concat() {
        let expected = ICFPString::from_str("#$".chars().collect()).unwrap();
        test_sequence("B. S# S$", Node::String(expected));
    }

    #[test]
    fn test_take() {
        let expected = ICFPString::from_str("#a".chars().collect()).unwrap();
        test_sequence("BT I# S#agc4gs", Node::String(expected));
    }

    #[test]
    fn test_drop() {
        let expected = ICFPString::from_str("gc4gs".chars().collect()).unwrap();
        test_sequence("BD I# S#agc4gs", Node::String(expected));
    }

    #[test]
    fn test_if() {
        test_sequence("? T I# I$", Node::Integer(2));
        test_sequence("? F I# I$", Node::Integer(3));
        test_sequence(
            "? B> I# I$ S9%3 S./",
            Node::String(ICFPString::from_str("./".chars().collect()).unwrap()),
        );
    }

    #[test]
    fn test_lambda_apply() {
        test_sequence("B$ L# B$ L\" B+ v\" v\" B* I$ I# v8", Node::Integer(12));
    }
}
