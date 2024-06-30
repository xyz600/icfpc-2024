use std::collections::{HashMap, HashSet, VecDeque};

use super::{
    icfpstring::ICFPString,
    tokenizer::{self, BinaryOpecode, TokenType, UnaryOpecode},
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
    Lazy(u32, u32),
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
            (Node::Lazy(_, i1), Node::Lazy(_, i2)) => i1 == i2,
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
            | Node::Variable(id, _)
            | Node::Lazy(id, _) => *id,
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
            | Node::Variable(id, _)
            | Node::Lazy(id, _) => id,
        }
    }
}

pub struct NodeFactory {
    node_id: u32,
    var_id: u32,
}

impl NodeFactory {
    pub fn new() -> NodeFactory {
        NodeFactory {
            node_id: 10,
            var_id: 10,
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

    fn lazy_node(&mut self, var_id: u32) -> Node {
        Node::Lazy(self.get_node_id(), var_id)
    }
}

// node 以下の変数名を unique に変更する
// 最初に呼ばれるだけのやつなので、lazy は含まれないと思ってよい。
pub fn alpha_convert(node: &mut Node, parser_state: &mut ParserState) {
    match node {
        Node::Boolean(_, _) | Node::Integer(_, _) | Node::String(_, _) | Node::Variable(_, _) => {}
        Node::Unary(_, _, child) => alpha_convert(child, parser_state),
        Node::Binary(_, _, child1, child2) => {
            alpha_convert(child1, parser_state);
            alpha_convert(child2, parser_state);
        }
        Node::If(_, pred, first, second) => {
            alpha_convert(pred, parser_state);
            alpha_convert(first, parser_state);
            alpha_convert(second, parser_state)
        }
        Node::Lambda(_, var_id, child) => {
            let new_var_id = parser_state.node_factory.get_var_id();
            // FIXME: 高速化
            replace_var_id_state(parser_state, var_id, new_var_id);
            replace_var_id(child, var_id, new_var_id);
            alpha_convert(child, parser_state);
        }
        Node::Lazy(_, _) => {
            unreachable!("Lazy should not be appeared");
        }
    }
}

fn replace_var_id_state(parser_state: &mut ParserState, from: &u32, to: u32) {
    for (_, node) in parser_state.cache_table.iter_mut() {
        replace_var_id(node, from, to);
    }
}

fn replace_var_id(node: &mut Node, from: &u32, to: u32) {
    match node {
        Node::Boolean(_, _) | Node::Integer(_, _) | Node::String(_, _) => {}
        Node::Unary(_, _, child) => replace_var_id(child, from, to),
        Node::Binary(_, _, child1, child2) => {
            replace_var_id(child1, from, to);
            replace_var_id(child2, from, to);
        }
        Node::If(_, pred, first, second) => {
            replace_var_id(pred, from, to);
            replace_var_id(first, from, to);
            replace_var_id(second, from, to);
        }
        Node::Lambda(_, var_id, child) => {
            if var_id == from {
                *var_id = to;
            }
            replace_var_id(child, from, to);
        }
        Node::Variable(_, var_id) => {
            if var_id == from {
                *var_id = to;
            }
        }
        Node::Lazy(_, _) => {
            unreachable!("Lazy should not be appeared");
        }
    }
}

fn construct_node(
    parser_state: &mut ParserState,
    token_stream: &mut VecDeque<TokenType>,
) -> Result<Node, ParseError> {
    if let Some(token) = token_stream.pop_front() {
        let node = match token {
            TokenType::Boolean(b) => parser_state.node_factory.boolean_node(b),
            TokenType::Integer(i) => parser_state.node_factory.integer_node(i),
            TokenType::String(s) => parser_state.node_factory.string_node(s),
            TokenType::Unary(opcode) => {
                let operand = construct_node(parser_state, token_stream)?;
                parser_state.node_factory.unary_node(opcode, operand)
            }
            TokenType::Binary(opcode) => {
                let operand1 = construct_node(parser_state, token_stream)?;
                let operand2 = construct_node(parser_state, token_stream)?;
                parser_state
                    .node_factory
                    .binary_node(opcode, operand1, operand2)
            }
            TokenType::If => {
                let operand1 = construct_node(parser_state, token_stream)?;
                let operand2 = construct_node(parser_state, token_stream)?;
                let operand3 = construct_node(parser_state, token_stream)?;
                parser_state
                    .node_factory
                    .if_node(operand1, operand2, operand3)
            }
            TokenType::Lambda(i) => {
                let operand = construct_node(parser_state, token_stream)?;
                parser_state.node_factory.lambda_node(i, operand)
            }
            TokenType::Variable(i) => parser_state.node_factory.variable_node(i),
        };
        Ok(node)
    } else {
        Err(ParseError::CannotFindNextToken)
    }
}

pub fn parse(input: String) -> Result<Node, ParseError> {
    let mut parser_state = ParserState::new();
    let token_list = tokenizer::tokenize(input)?;
    let mut queue = VecDeque::from_iter(token_list);
    let mut ast = construct_node(&mut parser_state, &mut queue)?;
    alpha_convert(&mut ast, &mut parser_state);
    let ast = evaluate_once(&mut parser_state, ast)?;
    Ok(ast)
}

// apply をするために variable(var_id) を node で置換する
pub fn substitute(root: &mut Node, var_id: u32, node: Node, parser_state: &mut ParserState) {
    fn substitute_inner(target: &mut Node, var_id: u32, node_factory: &mut NodeFactory) {
        match target {
            Node::Boolean(_, _) | Node::Integer(_, _) | Node::String(_, _) => {}
            Node::Unary(_, _, child) => substitute_inner(child, var_id, node_factory),
            Node::Binary(_, _, child1, child2) => {
                substitute_inner(child1, var_id, node_factory);
                substitute_inner(child2, var_id, node_factory);
            }
            Node::If(_, pred, first, second) => {
                substitute_inner(pred, var_id, node_factory);
                substitute_inner(first, var_id, node_factory);
                substitute_inner(second, var_id, node_factory);
            }
            Node::Lambda(_, ch_var_id, child) => {
                // 同名の束縛変数がある場合は置換しない
                if var_id != *ch_var_id {
                    substitute_inner(child, var_id, node_factory);
                }
            }
            Node::Variable(_, ch_var_id) => {
                if var_id == *ch_var_id {
                    // 対象の変数について、clone した結果を置く代わりに lazy を置いとく
                    *target = node_factory.lazy_node(var_id);
                }
            }
            Node::Lazy(_, var_id) => {}
        }
    }

    // FIXME: clone 消せる？
    parser_state.cache_table.insert(var_id, node);
    substitute_inner(root, var_id, &mut parser_state.node_factory);

    for (var_id, node) in parser_state.cache_table.iter_mut() {
        substitute_inner(node, *var_id, &mut parser_state.node_factory);
    }
}

pub fn evaluate_once(parser_state: &mut ParserState, node: Node) -> Result<Node, ParseError> {
    match node {
        // 値の場合はそのまま返す
        Node::Boolean(_, _) | Node::Integer(_, _) | Node::String(_, _) | Node::Variable(_, _) => {
            Ok(node)
        }
        Node::Unary(node_id, opcode, child) => {
            let child = evaluate_once(parser_state, *child)?;
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
            let child1 = evaluate_once(parser_state, *child1)?;
            let child2 = evaluate_once(parser_state, *child2)?;

            match opcode {
                BinaryOpecode::Add => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(parser_state.node_factory.integer_node(i1 + i2))
                    }
                    _ => Ok(parser_state.node_factory.binary_node(
                        BinaryOpecode::Add,
                        child1,
                        child2,
                    )),
                },
                BinaryOpecode::Sub => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(parser_state.node_factory.integer_node(i1 - i2))
                    }
                    _ => Ok(parser_state.node_factory.binary_node(
                        BinaryOpecode::Sub,
                        child1,
                        child2,
                    )),
                },
                BinaryOpecode::Mul => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(parser_state.node_factory.integer_node(i1 * i2))
                    }
                    _ => Ok(parser_state.node_factory.binary_node(
                        BinaryOpecode::Mul,
                        child1,
                        child2,
                    )),
                },
                BinaryOpecode::Div => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(parser_state.node_factory.integer_node(i1 / i2))
                    }
                    _ => Ok(parser_state.node_factory.binary_node(
                        BinaryOpecode::Div,
                        child1,
                        child2,
                    )),
                },
                BinaryOpecode::Modulo => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(Node::Integer(node_id, i1 % i2))
                    }
                    _ => Ok(parser_state.node_factory.binary_node(
                        BinaryOpecode::Modulo,
                        child1,
                        child2,
                    )),
                },
                BinaryOpecode::IntegerLarger => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(parser_state.node_factory.boolean_node(i1 < i2))
                    }
                    _ => Ok(parser_state.node_factory.binary_node(
                        BinaryOpecode::IntegerLarger,
                        child1,
                        child2,
                    )),
                },
                BinaryOpecode::IntegerSmaller => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(parser_state.node_factory.boolean_node(i1 > i2))
                    }
                    _ => Ok(parser_state.node_factory.binary_node(
                        BinaryOpecode::IntegerSmaller,
                        child1,
                        child2,
                    )),
                },
                BinaryOpecode::Equal => match (&child1, &child2) {
                    (Node::Integer(_, i1), Node::Integer(_, i2)) => {
                        Ok(parser_state.node_factory.boolean_node(i1 == i2))
                    }
                    (Node::String(_, s1), Node::String(_, s2)) => {
                        Ok(parser_state.node_factory.boolean_node(s1 == s2))
                    }
                    (Node::Boolean(_, b1), Node::Boolean(_, b2)) => {
                        Ok(parser_state.node_factory.boolean_node(b1 == b2))
                    }
                    _ => Ok(parser_state.node_factory.binary_node(
                        BinaryOpecode::Equal,
                        child1,
                        child2,
                    )),
                },
                BinaryOpecode::Or => match (&child1, &child2) {
                    (Node::Boolean(_, b1), Node::Boolean(_, b2)) => {
                        Ok(parser_state.node_factory.boolean_node(*b1 || *b2))
                    }
                    _ => {
                        Ok(parser_state
                            .node_factory
                            .binary_node(BinaryOpecode::Or, child1, child2))
                    }
                },
                BinaryOpecode::And => match (&child1, &child2) {
                    (Node::Boolean(_, b1), Node::Boolean(_, b2)) => {
                        Ok(parser_state.node_factory.boolean_node(*b1 && *b2))
                    }
                    _ => Ok(parser_state.node_factory.binary_node(
                        BinaryOpecode::And,
                        child1,
                        child2,
                    )),
                },
                BinaryOpecode::StrConcat => match (&child1, &child2) {
                    (Node::String(_, s1), Node::String(_, s2)) => {
                        Ok(parser_state.node_factory.string_node(s1.concat(&s2)))
                    }
                    _ => Ok(parser_state.node_factory.binary_node(
                        BinaryOpecode::StrConcat,
                        child1,
                        child2,
                    )),
                },
                BinaryOpecode::TakeStr => match (&child1, &child2) {
                    (Node::Integer(_, i), Node::String(_, s)) => {
                        Ok(parser_state.node_factory.string_node(s.take(*i as usize)))
                    }
                    _ => Ok(parser_state.node_factory.binary_node(
                        BinaryOpecode::TakeStr,
                        child1,
                        child2,
                    )),
                },
                BinaryOpecode::DropStr => match (&child1, &child2) {
                    (Node::Integer(_, i), Node::String(_, s)) => {
                        Ok(parser_state.node_factory.string_node(s.drop(*i as usize)))
                    }
                    _ => Ok(parser_state.node_factory.binary_node(
                        BinaryOpecode::DropStr,
                        child1,
                        child2,
                    )),
                },
                BinaryOpecode::Apply => match child1 {
                    Node::Lambda(_, var_id, child) => {
                        let mut child = *child;
                        substitute(&mut child, var_id, child2, parser_state);
                        Ok(child)
                    }
                    _ => Ok(parser_state.node_factory.binary_node(
                        BinaryOpecode::Apply,
                        child1,
                        child2,
                    )),
                },
            }
        }
        Node::If(_, pred, first, second) => {
            let pred = evaluate_once(parser_state, *pred)?;
            match pred {
                Node::Boolean(_, b) => {
                    if b {
                        evaluate_once(parser_state, *first)
                    } else {
                        evaluate_once(parser_state, *second)
                    }
                }
                _ => Ok(parser_state.node_factory.if_node(pred, *first, *second)),
            }
        }
        Node::Lambda(_, i, child) => {
            let child = evaluate_once(parser_state, *child)?;
            Ok(parser_state.node_factory.lambda_node(i, child))
        }
        Node::Lazy(_, var_id) => {
            if let Some(node) = parser_state.cache_table.get(&var_id) {
                Ok(node.clone())
            } else {
                unreachable!("Lazy node should be replaced by its value")
            }
        }
    }
}

pub struct ParserState {
    node_factory: NodeFactory,
    cache_table: HashMap<u32, Node>,
}

impl ParserState {
    pub fn new() -> ParserState {
        ParserState {
            node_factory: NodeFactory::new(),
            cache_table: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_sequence(input: &str, expected: Node) {
        let result = parse(input.to_string()).unwrap();
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
