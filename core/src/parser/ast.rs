use std::collections::VecDeque;

use super::{
    icfpstring::ICFPString,
    tokenizer::{BinaryOpecode, TokenType, UnaryOpecode},
    ParseError,
};

#[derive(Clone, Debug)]
pub enum Node {
    Boolean(u32, bool),
    Integer(u32, i64),
    String(u32, ICFPString),
    Unary(u32, UnaryOpecode, Box<Node>),
    Binary(u32, BinaryOpecode, Box<Node>, Box<Node>),
    If(u32, Box<Node>, Box<Node>, Box<Node>),
    Lambda(u32, i64, Box<Node>),
    Variable(u32, i64),
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Node::Boolean(_, b1), Node::Boolean(_, b2)) => b1 == b2,
            (Node::Integer(_, i1), Node::Integer(_, i2)) => i1 == i2,
            (Node::String(_, s1), Node::String(_, s2)) => s1 == s2,
            (Node::Unary(_, opcode1, child1), Node::Unary(_, opcode2, child2)) => {
                opcode1 == opcode2 && child1 == child2
            }
            (
                Node::Binary(_, opcode1, child11, child12),
                Node::Binary(_, opcode2, child21, child22),
            ) => opcode1 == opcode2 && child11 == child21 && child12 == child22,
            (Node::If(_, pred1, first1, second1), Node::If(_, pred2, first2, second2)) => {
                pred1 == pred2 && first1 == first2 && second1 == second2
            }
            (Node::Lambda(_, i1, child1), Node::Lambda(_, i2, child2)) => {
                i1 == i2 && child1 == child2
            }
            (Node::Variable(_, i1), Node::Variable(_, i2)) => i1 == i2,
            _ => false,
        }
    }
}

impl Node {
    fn id(&self) -> u32 {
        match self {
            Node::Boolean(id, _)
            | Node::Integer(id, _)
            | Node::String(id, _)
            | Node::Unary(id, _, _)
            | Node::Binary(id, _, _, _)
            | Node::If(id, _, _, _)
            | Node::Lambda(id, _, _)
            | Node::Variable(id, _) => *id,
        }
    }

    pub fn visit(&self, f: &mut impl FnMut(&Node)) {
        f(&self);
        match self {
            Node::Boolean(_, _)
            | Node::Integer(_, _)
            | Node::String(_, _)
            | Node::Variable(_, _) => {}
            Node::Unary(_, _, child) => child.visit(f),
            Node::Binary(_, _, child1, child2) => {
                child1.visit(f);
                child2.visit(f);
            }
            Node::If(_, pred, first, second) => {
                pred.visit(f);
                first.visit(f);
                second.visit(f);
            }
            Node::Lambda(_, _, child) => child.visit(f),
        }
    }

    pub fn visit_mut(&mut self, f: &mut impl FnMut(&mut Node)) {
        f(self);
        match self {
            Node::Boolean(_, _)
            | Node::Integer(_, _)
            | Node::String(_, _)
            | Node::Variable(_, _) => {}
            Node::Unary(_, _, child) => child.visit_mut(f),
            Node::Binary(_, _, child1, child2) => {
                child1.visit_mut(f);
                child2.visit_mut(f);
            }
            Node::If(_, pred, first, second) => {
                pred.visit_mut(f);
                first.visit_mut(f);
                second.visit_mut(f);
            }
            Node::Lambda(_, _, child) => child.visit_mut(f),
        }
    }

    pub fn to_dot_string(&self) -> String {
        let mut dot = String::new();
        dot.push_str("digraph G {\n");
        self.visit(&mut |node| match node {
            Node::Boolean(id, b) => {
                dot.push_str(&format!("{} [label=\"{}\"];\n", id, b));
            }
            Node::Integer(id, i) => {
                dot.push_str(&format!("{} [label=\"{}\"];\n", id, i));
            }
            Node::String(id, s) => {
                dot.push_str(&format!("{} [label=\"{}\"];\n", id, s));
            }
            Node::Unary(id, opcode, _) => {
                dot.push_str(&format!("{} [label=\"{:?}\"];\n", id, opcode));
            }
            Node::Binary(id, opcode, _, _) => {
                dot.push_str(&format!("{} [label=\"{:?}\"];\n", id, opcode));
            }
            Node::If(id, _, _, _) => {
                dot.push_str(&format!("{} [label=\"If\"];\n", id));
            }
            Node::Lambda(id, i, _) => {
                dot.push_str(&format!("{} [label=\"Lambda({})\"];\n", id, i));
            }
            Node::Variable(id, i) => {
                dot.push_str(&format!("{} [label=\"Variable({})\"];\n", id, i));
            }
        });
        self.visit(&mut |node| match node {
            Node::Boolean(id, _)
            | Node::Integer(id, _)
            | Node::String(id, _)
            | Node::Variable(id, _) => {}
            Node::Unary(id, _, child) => {
                dot.push_str(&format!("{} -> {};\n", id, id));
            }
            Node::Binary(id, _, child1, child2) => {
                dot.push_str(&format!("{} -> {};\n", id, child1.id()));
                dot.push_str(&format!("{} -> {};\n", id, child2.id()));
            }
            Node::If(id, pred, first, second) => {
                dot.push_str(&format!("{} -> {};\n", id, pred.id()));
                dot.push_str(&format!("{} -> {};\n", id, first.id()));
                dot.push_str(&format!("{} -> {};\n", id, second.id()));
            }
            Node::Lambda(id, var, expr) => {
                dot.push_str(&format!("{} -> {};\n", id, expr.id()));
            }
        });
        dot.push_str("}\n");
        dot
    }
}

pub fn parse(token_stream: &mut VecDeque<TokenType>, id: &mut u32) -> Result<Node, ParseError> {
    let node_id = *id;
    *id += 1;
    if let Some(token) = token_stream.pop_front() {
        let node = match &token {
            TokenType::Boolean(b) => Node::Boolean(node_id, *b),
            TokenType::Integer(i) => Node::Integer(node_id, *i),
            TokenType::String(s) => Node::String(node_id, s.clone()),
            TokenType::Unary(opcode) => {
                let operand = parse(token_stream, id)?;
                Node::Unary(node_id, *opcode, Box::new(operand))
            }
            TokenType::Binary(opcode) => {
                let operand1 = parse(token_stream, id)?;
                let operand2 = parse(token_stream, id)?;
                Node::Binary(node_id, *opcode, Box::new(operand1), Box::new(operand2))
            }
            TokenType::If => {
                let operand1 = parse(token_stream, id)?;
                let operand2 = parse(token_stream, id)?;
                let operand3 = parse(token_stream, id)?;
                Node::If(
                    node_id,
                    Box::new(operand1),
                    Box::new(operand2),
                    Box::new(operand3),
                )
            }
            TokenType::Lambda(i) => {
                let operand = parse(token_stream, id)?;
                Node::Lambda(node_id, *i, Box::new(operand))
            }
            TokenType::Variable(i) => Node::Variable(node_id, *i),
        };
        Ok(node)
    } else {
        Err(ParseError::CannotFindNextToken)
    }
}

/// NOTE: node id を使いまわしているけど、本当に良いかは確認した方がいいかもしれない
/// 簡約が絶対できないときに Runtime Error を起こしたいんだけど、面倒すぎて一旦保留
pub fn evaluate(node: Node) -> Result<Node, ParseError> {
    match node {
        // 値の場合はそのまま返す
        Node::Boolean(_, _) | Node::Integer(_, _) | Node::String(_, _) | Node::Variable(_, _) => {
            Ok(node)
        }
        Node::Unary(id, opcode, child) => {
            let child = evaluate(*child)?;
            match opcode {
                UnaryOpecode::Negate => match child {
                    Node::Integer(_, i) => Ok(Node::Integer(id, -i)),
                    _ => Err(ParseError::InvalidUnaryNegateOperand),
                },
                UnaryOpecode::Not => match child {
                    Node::Boolean(_, b) => Ok(Node::Boolean(id, !b)),
                    _ => Err(ParseError::InvalidUnaryNotOperand),
                },
                UnaryOpecode::StrToInt => match child {
                    Node::String(_, s) => Ok(Node::Integer(id, s.to_i64())),
                    _ => Err(ParseError::InvalidUnaryStrToIntOperand),
                },
                UnaryOpecode::IntToStr => match child {
                    Node::Integer(_, i) => Ok(Node::String(id, ICFPString::from_i64(i))),
                    _ => Err(ParseError::InvalidUnaryIntToStrOperand),
                },
            }
        }
        Node::Binary(id, opcode, child1, child2) => {
            let child1 = evaluate(*child1)?;
            let child2 = evaluate(*child2)?;

            match opcode {
                BinaryOpecode::Apply => {
                    match child1 {
                        Node::Lambda(_, i, child) => {
                            let mut child = *child;
                            child.visit_mut(&mut |node| {
                                if let Node::Variable(_, j) = node {
                                    if *j == i {
                                        // FIXME: unique id generator を使わないといけなさそうだ…
                                        *node = child2.clone();
                                    }
                                }
                            });
                            Ok(evaluate(child)?)
                        }
                        _ => Ok(Node::Binary(
                            id,
                            BinaryOpecode::Apply,
                            Box::new(child1),
                            Box::new(child2),
                        )),
                    }
                }
                BinaryOpecode::Add => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => Ok(Node::Integer(id, i1 + i2)),
                    _ => Ok(Node::Binary(
                        id,
                        BinaryOpecode::Add,
                        Box::new(child1),
                        Box::new(child2),
                    )),
                },
                BinaryOpecode::Sub => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => Ok(Node::Integer(id, i1 - i2)),
                    _ => Ok(Node::Binary(
                        id,
                        BinaryOpecode::Sub,
                        Box::new(child1),
                        Box::new(child2),
                    )),
                },
                BinaryOpecode::Mul => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => Ok(Node::Integer(id, i1 * i2)),
                    _ => Ok(Node::Binary(
                        id,
                        BinaryOpecode::Mul,
                        Box::new(child1),
                        Box::new(child2),
                    )),
                },
                BinaryOpecode::Div => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => Ok(Node::Integer(id, i1 / i2)),
                    _ => Ok(Node::Binary(
                        id,
                        BinaryOpecode::Div,
                        Box::new(child1),
                        Box::new(child2),
                    )),
                },
                BinaryOpecode::Modulo => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => Ok(Node::Integer(id, i1 % i2)),
                    _ => Ok(Node::Binary(
                        id,
                        BinaryOpecode::Modulo,
                        Box::new(child1),
                        Box::new(child2),
                    )),
                },
                BinaryOpecode::IntegerLarger => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => Ok(Node::Boolean(id, i1 < i2)),
                    _ => Ok(Node::Binary(
                        id,
                        BinaryOpecode::IntegerLarger,
                        Box::new(child1),
                        Box::new(child2),
                    )),
                },
                BinaryOpecode::IntegerSmaller => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => Ok(Node::Boolean(id, i1 > i2)),
                    _ => Ok(Node::Binary(
                        id,
                        BinaryOpecode::IntegerSmaller,
                        Box::new(child1),
                        Box::new(child2),
                    )),
                },
                BinaryOpecode::Equal => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => Ok(Node::Boolean(id, i1 == i2)),
                    (Node::String(_, s1), Node::String(_, s2)) => Ok(Node::Boolean(id, s1 == s2)),
                    (Node::Boolean(_, b1), Node::Boolean(_, b2)) => Ok(Node::Boolean(id, b1 == b2)),
                    _ => Ok(Node::Binary(
                        id,
                        BinaryOpecode::Equal,
                        Box::new(child1),
                        Box::new(child2),
                    )),
                },
                BinaryOpecode::Or => match (&child1, &child2) {
                    (Node::Boolean(_, b1), Node::Boolean(_, b2)) => {
                        Ok(Node::Boolean(id, *b1 || *b2))
                    }
                    _ => Ok(Node::Binary(
                        id,
                        BinaryOpecode::Or,
                        Box::new(child1),
                        Box::new(child2),
                    )),
                },
                BinaryOpecode::And => match (&child1, &child2) {
                    (Node::Boolean(_, b1), Node::Boolean(_, b2)) => {
                        Ok(Node::Boolean(id, *b1 && *b2))
                    }
                    _ => Ok(Node::Binary(
                        id,
                        BinaryOpecode::And,
                        Box::new(child1),
                        Box::new(child2),
                    )),
                },
                BinaryOpecode::StrConcat => match (&child1, &child2) {
                    (Node::String(_, s1), Node::String(_, s2)) => {
                        Ok(Node::String(id, s1.concat(&s2)))
                    }
                    _ => Ok(Node::Binary(
                        id,
                        BinaryOpecode::StrConcat,
                        Box::new(child1),
                        Box::new(child2),
                    )),
                },
                BinaryOpecode::TakeStr => match (&child1, &child2) {
                    (Node::Integer(_, i), Node::String(_, s)) => {
                        Ok(Node::String(id, s.take(*i as usize)))
                    }
                    _ => Ok(Node::Binary(
                        id,
                        BinaryOpecode::TakeStr,
                        Box::new(child1),
                        Box::new(child2),
                    )),
                },
                BinaryOpecode::DropStr => match (&child1, &child2) {
                    (Node::Integer(_, i), Node::String(_, s)) => {
                        Ok(Node::String(id, s.drop(*i as usize)))
                    }
                    _ => Ok(Node::Binary(
                        id,
                        BinaryOpecode::DropStr,
                        Box::new(child1),
                        Box::new(child2),
                    )),
                },
            }
        }
        Node::If(id, pred, first, second) => {
            let pred = evaluate(*pred)?;
            match pred {
                Node::Boolean(_, b) => {
                    if b {
                        evaluate(*first)
                    } else {
                        evaluate(*second)
                    }
                }
                _ => {
                    let first = evaluate(*first)?;
                    let second = evaluate(*second)?;
                    Ok(Node::If(
                        id,
                        Box::new(pred),
                        Box::new(first),
                        Box::new(second),
                    ))
                }
            }
        }
        Node::Lambda(id, i, child) => {
            let child = evaluate(*child)?;
            Ok(Node::Lambda(id, i, Box::new(child)))
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
        let mut id = 0;
        let node = parse(&mut token_stream, &mut id).unwrap();
        eprintln!("before: ");
        eprintln!("{}", node.to_dot_string());
        eprintln!("{:?}", node);
        let result = evaluate(node).unwrap();
        assert_eq!(result, expected);
    }

    // testcase is generated from https://icfpcontest2024.github.io/icfp.html

    #[test]
    fn test_unary_negate() {
        test_sequence("U- I$", Node::Integer(0, -3));
    }

    #[test]
    fn test_unary_not() {
        test_sequence("U! B= S$ S$", Node::Boolean(0, false));
        test_sequence("U! B= I/ I$", Node::Boolean(0, true));
    }

    #[test]
    fn test_unary_strtoint() {
        test_sequence("U# S4%34", Node::Integer(0, 15818151));
    }

    #[test]
    fn test_add() {
        test_sequence("B+ I# I$", Node::Integer(0, 5));
    }

    #[test]
    fn test_sub() {
        test_sequence("B- I$ I#", Node::Integer(0, 1));
    }

    #[test]
    fn test_mul() {
        test_sequence("B* I# I$", Node::Integer(0, 6));
    }

    #[test]
    fn test_div() {
        test_sequence("B/ U- I( I#", Node::Integer(0, -3));
    }

    #[test]
    fn test_mod() {
        test_sequence("B% U- I( I#", Node::Integer(0, -1));
    }

    #[test]
    fn test_gt() {
        test_sequence("B< I$ I#", Node::Boolean(0, false));
        test_sequence("B< I# I$", Node::Boolean(0, true));
    }

    #[test]
    fn test_lt() {
        test_sequence("B> I$ I#", Node::Boolean(0, true));
        test_sequence("B> I# I$", Node::Boolean(0, false));
    }

    #[test]
    fn test_eq() {
        test_sequence("B= I$ I#", Node::Boolean(0, false));
        test_sequence("B= I$ B+ I# I\"", Node::Boolean(0, true));

        test_sequence("B= S# S#", Node::Boolean(0, true));
        test_sequence("B= S# S$", Node::Boolean(0, false));

        test_sequence("B= T B= F F", Node::Boolean(0, true));
        test_sequence("B= F B= F F", Node::Boolean(0, false));
    }

    #[test]
    fn test_and() {
        test_sequence("B& T F", Node::Boolean(0, false));
        test_sequence("B& T T", Node::Boolean(0, true));
    }

    #[test]
    fn test_or() {
        test_sequence("B| T F", Node::Boolean(0, true));
        test_sequence("B| F F", Node::Boolean(0, false));
    }

    #[test]
    fn test_concat() {
        let expected = ICFPString::from_rawstr("#$").unwrap();
        test_sequence("B. S# S$", Node::String(0, expected));
    }

    #[test]
    fn test_take() {
        let expected = ICFPString::from_rawstr("#a").unwrap();
        test_sequence("BT I# S#agc4gs", Node::String(0, expected));
    }

    #[test]
    fn test_drop() {
        let expected = ICFPString::from_rawstr("gc4gs").unwrap();
        test_sequence("BD I# S#agc4gs", Node::String(0, expected));
    }

    #[test]
    fn test_if() {
        test_sequence("? T I# I$", Node::Integer(0, 2));
        test_sequence("? F I# I$", Node::Integer(0, 3));
        test_sequence(
            "? B> I# I$ S9%3 S./",
            Node::String(0, ICFPString::from_rawstr("./").unwrap()),
        );
    }

    #[test]
    fn test_lambda_apply1() {
        test_sequence("B$ L# B$ L\" B+ v\" v\" B* I$ I# v8", Node::Integer(0, 12));
    }

    #[test]
    fn test_lambda_apply2() {
        test_sequence(
            "B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK",
            Node::String(0, ICFPString::from_rawstr("B%,,/}Q/2,$_").unwrap()),
        );
    }

    // #[test]
    fn test_lambda_apply3() {
        test_sequence("", Node::Integer(0, 16));
    }

    // #[test]
    fn test_lambda_apply4() {
        test_sequence(
            "B$ B$ L\" B$ L# B$ v\" B$ v# v# L# B$ v\" B$ v# v# L\" L# ? B= v# I! I\" B$ L$ B+ B$ v\" v$ B$ v\" v$ B- v# I\" I%",
            Node::Integer(0, 16),
        );
    }
}
