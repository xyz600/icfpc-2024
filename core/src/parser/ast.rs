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
    Lambda(u32, u32, Box<Node>),
    Variable(u32, u32),
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
    pub fn len(&self) -> usize {
        let mut size = 0;
        self.visit(&mut |_node| size += 1);
        size
    }

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

    fn id_mut(&mut self) -> &mut u32 {
        match self {
            Node::Boolean(id, _)
            | Node::Integer(id, _)
            | Node::String(id, _)
            | Node::Unary(id, _, _)
            | Node::Binary(id, _, _, _)
            | Node::If(id, _, _, _)
            | Node::Lambda(id, _, _)
            | Node::Variable(id, _) => id,
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

    pub fn substitute(&mut self, i: u32, node: &Node, node_factory: &mut NodeFactory) {
        match self {
            Node::Boolean(_, _) | Node::Integer(_, _) | Node::String(_, _) => {}
            Node::Unary(_, _, child) => child.substitute(i, node, node_factory),
            Node::Binary(_, _, child1, child2) => {
                child1.substitute(i, node, node_factory);
                child2.substitute(i, node, node_factory);
            }
            Node::If(_, pred, first, second) => {
                pred.substitute(i, node, node_factory);
                first.substitute(i, node, node_factory);
                second.substitute(i, node, node_factory);
            }
            Node::Lambda(_, j, child) => {
                // 同名の束縛変数がある場合は置換しない
                if i != *j {
                    child.substitute(i, node, node_factory);
                }
            }
            Node::Variable(_, j) => {
                if i == *j {
                    let mut new_node = node.clone();
                    node_factory.replace_var_id(&mut new_node);
                    *self = new_node;
                }
            }
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
            Node::Boolean(_, _)
            | Node::Integer(_, _)
            | Node::String(_, _)
            | Node::Variable(_, _) => {}
            Node::Unary(id, _, _) => {
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
            Node::Lambda(id, _, expr) => {
                dot.push_str(&format!("{} -> {};\n", id, expr.id()));
            }
        });
        dot.push_str("}\n");
        dot
    }
}

pub struct NodeFactory {
    node_id: u32,
    var_id: u32,
}

impl NodeFactory {
    pub fn new() -> NodeFactory {
        NodeFactory {
            node_id: 16384,
            var_id: 16384,
        }
    }

    fn get_node_id(&mut self) -> u32 {
        let ret = self.node_id;
        self.node_id += 1;
        ret
    }

    fn get_var_id(&mut self) -> u32 {
        let ret = self.var_id;
        self.var_id += 1;
        ret
    }

    fn recur(&mut self, target_var_id: u32, new_var_id: u32, node: &mut Node) {
        match node {
            Node::Boolean(_, _) | Node::Integer(_, _) | Node::String(_, _) => {}
            Node::Unary(_, _, child) => self.recur(target_var_id, new_var_id, child),
            Node::Binary(_, _, child1, child2) => {
                self.recur(target_var_id, new_var_id, child1);
                self.recur(target_var_id, new_var_id, child2);
            }
            Node::If(_, pred, first, second) => {
                self.recur(target_var_id, new_var_id, pred);
                self.recur(target_var_id, new_var_id, first);
                self.recur(target_var_id, new_var_id, second);
            }
            Node::Lambda(_, var_id, child) => {
                // 同名の束縛変数がある場合は潜ると壊れる
                if *var_id != target_var_id {
                    self.recur(target_var_id, new_var_id, child);
                }
            }
            Node::Variable(_, var_id) => {
                if *var_id == target_var_id {
                    *var_id = new_var_id;
                }
            }
        }
    }

    fn replace_var_id(&mut self, node: &mut Node) {
        let new_var_id = self.get_var_id();

        node.visit_mut(&mut |node| {
            let new_node_id = self.get_node_id();
            *node.id_mut() = new_node_id;
        });

        match node {
            Node::Lambda(_, var_id, child) => {
                self.recur(*var_id, new_var_id, child);
                *var_id = new_var_id;
            }
            _ => {}
        }
    }

    fn boolean_node(&mut self, b: bool) -> Node {
        Node::Boolean(self.get_node_id(), b)
    }

    fn integer_node(&mut self, i: i64) -> Node {
        Node::Integer(self.get_node_id(), i)
    }

    fn string_node(&mut self, s: ICFPString) -> Node {
        Node::String(self.get_node_id(), s)
    }

    fn unary_node(&mut self, opcode: UnaryOpecode, child: Node) -> Node {
        Node::Unary(self.get_node_id(), opcode, Box::new(child))
    }

    fn binary_node(&mut self, opcode: BinaryOpecode, child1: Node, child2: Node) -> Node {
        Node::Binary(
            self.get_node_id(),
            opcode,
            Box::new(child1),
            Box::new(child2),
        )
    }

    fn if_node(&mut self, pred: Node, first: Node, second: Node) -> Node {
        Node::If(
            self.get_node_id(),
            Box::new(pred),
            Box::new(first),
            Box::new(second),
        )
    }

    fn lambda_node(&mut self, var_id: u32, expr: Node) -> Node {
        Node::Lambda(self.get_node_id(), var_id, Box::new(expr))
    }

    fn variable_node(&mut self, var_id: u32) -> Node {
        Node::Variable(self.get_node_id(), var_id)
    }
}

pub fn parse(
    token_stream: &mut VecDeque<TokenType>,
    node_factory: &mut NodeFactory,
) -> Result<Node, ParseError> {
    if let Some(token) = token_stream.pop_front() {
        let node = match token {
            TokenType::Boolean(b) => node_factory.boolean_node(b),
            TokenType::Integer(i) => node_factory.integer_node(i),
            TokenType::String(s) => node_factory.string_node(s),
            TokenType::Unary(opcode) => {
                let operand = parse(token_stream, node_factory)?;
                node_factory.unary_node(opcode, operand)
            }
            TokenType::Binary(opcode) => {
                let operand1 = parse(token_stream, node_factory)?;
                let operand2 = parse(token_stream, node_factory)?;
                node_factory.binary_node(opcode, operand1, operand2)
            }
            TokenType::If => {
                let operand1 = parse(token_stream, node_factory)?;
                let operand2 = parse(token_stream, node_factory)?;
                let operand3 = parse(token_stream, node_factory)?;
                node_factory.if_node(operand1, operand2, operand3)
            }
            TokenType::Lambda(i) => {
                let operand = parse(token_stream, node_factory)?;
                node_factory.lambda_node(i, operand)
            }
            TokenType::Variable(i) => node_factory.variable_node(i),
        };
        Ok(node)
    } else {
        Err(ParseError::CannotFindNextToken)
    }
}

/// NOTE: node id を使いまわしているけど、本当に良いかは確認した方がいいかもしれない
/// 簡約が絶対できないときに Runtime Error を起こしたいんだけど、面倒すぎて一旦保留
pub fn evaluate(node: Node, node_factory: &mut NodeFactory) -> Result<Node, ParseError> {
    let mut node = node;
    for _i in 0..10_000_000 {
        let new_node = evaluate_once(node.clone(), node_factory)?;
        if new_node == node {
            return Ok(new_node);
        }
        node = new_node;
    }
    Ok(node)
}

pub fn evaluate_once(node: Node, node_factory: &mut NodeFactory) -> Result<Node, ParseError> {
    match node {
        // 値の場合はそのまま返す
        Node::Boolean(_, _) | Node::Integer(_, _) | Node::String(_, _) | Node::Variable(_, _) => {
            Ok(node)
        }
        Node::Unary(node_id, opcode, child) => {
            let child = evaluate_once(*child, node_factory)?;
            match opcode {
                UnaryOpecode::Negate => match child {
                    Node::Integer(_, i) => Ok(Node::Integer(node_id, -i)),
                    _ => Ok(Node::Unary(node_id, UnaryOpecode::Negate, Box::new(child))),
                },
                UnaryOpecode::Not => match child {
                    Node::Boolean(_, b) => Ok(Node::Boolean(node_id, !b)),
                    _ => Ok(Node::Unary(node_id, UnaryOpecode::Not, Box::new(child))),
                },
                UnaryOpecode::StrToInt => match child {
                    Node::String(_, s) => Ok(Node::Integer(node_id, s.to_i64())),
                    _ => Ok(Node::Unary(
                        node_id,
                        UnaryOpecode::StrToInt,
                        Box::new(child),
                    )),
                },
                UnaryOpecode::IntToStr => match child {
                    Node::Integer(_, i) => Ok(Node::String(node_id, ICFPString::from_i64(i))),
                    _ => Ok(Node::Unary(
                        node_id,
                        UnaryOpecode::IntToStr,
                        Box::new(child),
                    )),
                },
            }
        }
        Node::Binary(node_id, opcode, child1, child2) => {
            let child1 = evaluate_once(*child1, node_factory)?;
            let child2 = evaluate_once(*child2, node_factory)?;

            match opcode {
                BinaryOpecode::Add => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(node_factory.integer_node(i1 + i2))
                    }
                    _ => Ok(node_factory.binary_node(BinaryOpecode::Add, child1, child2)),
                },
                BinaryOpecode::Sub => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(node_factory.integer_node(i1 - i2))
                    }
                    _ => Ok(node_factory.binary_node(BinaryOpecode::Sub, child1, child2)),
                },
                BinaryOpecode::Mul => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(node_factory.integer_node(i1 * i2))
                    }
                    _ => Ok(node_factory.binary_node(BinaryOpecode::Mul, child1, child2)),
                },
                BinaryOpecode::Div => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(node_factory.integer_node(i1 / i2))
                    }
                    _ => Ok(node_factory.binary_node(BinaryOpecode::Div, child1, child2)),
                },
                BinaryOpecode::Modulo => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(Node::Integer(node_id, i1 % i2))
                    }
                    _ => Ok(Node::Binary(
                        node_id,
                        BinaryOpecode::Modulo,
                        Box::new(child1),
                        Box::new(child2),
                    )),
                },
                BinaryOpecode::IntegerLarger => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(node_factory.boolean_node(i1 < i2))
                    }
                    _ => Ok(node_factory.binary_node(BinaryOpecode::IntegerLarger, child1, child2)),
                },
                BinaryOpecode::IntegerSmaller => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(node_factory.boolean_node(i1 > i2))
                    }
                    _ => {
                        Ok(node_factory.binary_node(BinaryOpecode::IntegerSmaller, child1, child2))
                    }
                },
                BinaryOpecode::Equal => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(node_factory.boolean_node(i1 == i2))
                    }
                    (Node::String(_, s1), Node::String(_, s2)) => {
                        Ok(node_factory.boolean_node(s1 == s2))
                    }
                    (Node::Boolean(_, b1), Node::Boolean(_, b2)) => {
                        Ok(node_factory.boolean_node(b1 == b2))
                    }
                    _ => Ok(node_factory.binary_node(BinaryOpecode::Equal, child1, child2)),
                },
                BinaryOpecode::Or => match (&child1, &child2) {
                    (Node::Boolean(_, b1), Node::Boolean(_, b2)) => {
                        Ok(node_factory.boolean_node(*b1 || *b2))
                    }
                    _ => Ok(node_factory.binary_node(BinaryOpecode::Or, child1, child2)),
                },
                BinaryOpecode::And => match (&child1, &child2) {
                    (Node::Boolean(_, b1), Node::Boolean(_, b2)) => {
                        Ok(node_factory.boolean_node(*b1 && *b2))
                    }
                    _ => Ok(node_factory.binary_node(BinaryOpecode::And, child1, child2)),
                },
                BinaryOpecode::StrConcat => match (&child1, &child2) {
                    (Node::String(_, s1), Node::String(_, s2)) => {
                        Ok(node_factory.string_node(s1.concat(&s2)))
                    }
                    _ => Ok(node_factory.binary_node(BinaryOpecode::StrConcat, child1, child2)),
                },
                BinaryOpecode::TakeStr => match (&child1, &child2) {
                    (Node::Integer(_, i), Node::String(_, s)) => {
                        Ok(node_factory.string_node(s.take(*i as usize)))
                    }
                    _ => Ok(node_factory.binary_node(BinaryOpecode::TakeStr, child1, child2)),
                },
                BinaryOpecode::DropStr => match (&child1, &child2) {
                    (Node::Integer(_, i), Node::String(_, s)) => {
                        Ok(node_factory.string_node(s.drop(*i as usize)))
                    }
                    _ => Ok(node_factory.binary_node(BinaryOpecode::DropStr, child1, child2)),
                },
                BinaryOpecode::Apply => match child1 {
                    // FIXME: substitute するタイミングで、変数を unique なものに書き換える
                    Node::Lambda(_, i, child) => {
                        let mut child = *child;
                        child.substitute(i, &child2, node_factory);
                        Ok(child)
                    }
                    _ => Ok(node_factory.binary_node(BinaryOpecode::Apply, child1, child2)),
                },
            }
        }
        Node::If(_, pred, first, second) => {
            let pred = evaluate_once(*pred, node_factory)?;
            match pred {
                Node::Boolean(_, b) => {
                    if b {
                        evaluate_once(*first, node_factory)
                    } else {
                        evaluate_once(*second, node_factory)
                    }
                }
                _ => {
                    let first = evaluate_once(*first, node_factory)?;
                    let second = evaluate_once(*second, node_factory)?;
                    Ok(node_factory.if_node(pred, first, second))
                }
            }
        }
        Node::Lambda(_, i, child) => {
            let child = evaluate_once(*child, node_factory)?;
            Ok(node_factory.lambda_node(i, child))
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
        let mut node_factory = NodeFactory::new();
        let node = parse(&mut token_stream, &mut node_factory).unwrap();
        let result = evaluate(node, &mut node_factory).unwrap();
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
        )
    }

    #[test]
    fn test_lambda_apply4() {
        test_sequence(
                    "B$ B$ L\" B$ L# B$ v\" B$ v# v# L# B$ v\" B$ v# v# L\" L# ? B= v# I! I\" B$ L$ B+ B$ v\" v$ B$ v\" v$ B- v# I\" I%",
                    Node::Integer(0, 16),
                )
    }
}
