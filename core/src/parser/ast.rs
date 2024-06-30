use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::{Index, IndexMut},
};

use super::{
    icfpstring::ICFPString,
    tokenizer::{self, BinaryOpecode, TokenType, UnaryOpecode},
    ParseError,
};

#[derive(Clone, Debug, PartialEq)]
pub enum NodeType {
    Boolean(bool),
    Integer(i64),
    String(ICFPString),
    Unary(UnaryOpecode, usize),
    Binary(BinaryOpecode, usize, usize),
    If(usize, usize, usize),
    Lambda(u32, usize),
    Variable(u32),
    Lazy(u32),
}

#[derive(Clone, Debug)]
pub struct Node {
    pub node_id: usize,
    pub node_type: NodeType,
}

impl Node {
    pub fn new(node_id: usize, node_type: NodeType) -> Node {
        Node { node_id, node_type }
    }
}

pub struct NodeFactory {
    node_id: usize,
    var_id: u32,
    node_buffer: Vec<Node>,
    root_id: usize,
}

impl NodeFactory {
    pub fn new() -> NodeFactory {
        NodeFactory {
            node_id: 0,
            var_id: 128,
            node_buffer: Vec::new(),
            root_id: 0,
        }
    }

    fn get_node_id(&mut self) -> usize {
        let ret = self.node_id;
        self.node_id += 1;
        ret
    }

    fn get_var_id(&mut self) -> u32 {
        let ret = self.var_id;
        self.var_id += 1;
        ret
    }

    fn boolean_node(&mut self, b: bool) -> usize {
        let new_node_id = self.get_node_id();
        self.node_buffer
            .push(Node::new(new_node_id, NodeType::Boolean(b)));
        self.node_buffer.len() - 1
    }

    fn integer_node(&mut self, i: i64) -> usize {
        let new_node_id = self.get_node_id();
        self.node_buffer
            .push(Node::new(new_node_id, NodeType::Integer(i)));
        self.node_buffer.len() - 1
    }

    fn string_node(&mut self, s: ICFPString) -> usize {
        let new_node_id = self.get_node_id();
        self.node_buffer
            .push(Node::new(new_node_id, NodeType::String(s)));
        self.node_buffer.len() - 1
    }

    fn unary_node(&mut self, opcode: UnaryOpecode, child_id: usize) -> usize {
        let new_node_id = self.get_node_id();
        self.node_buffer
            .push(Node::new(new_node_id, NodeType::Unary(opcode, child_id)));
        self.node_buffer.len() - 1
    }

    fn binary_node(&mut self, opcode: BinaryOpecode, child_id1: usize, child_id2: usize) -> usize {
        let new_node_id = self.get_node_id();
        self.node_buffer.push(Node::new(
            new_node_id,
            NodeType::Binary(opcode, child_id1, child_id2),
        ));
        self.node_buffer.len() - 1
    }

    fn if_node(&mut self, pred: usize, first: usize, second: usize) -> usize {
        let new_node_id = self.get_node_id();
        self.node_buffer
            .push(Node::new(new_node_id, NodeType::If(pred, first, second)));
        self.node_buffer.len() - 1
    }

    fn lambda_node(&mut self, var_id: u32, expr: usize) -> usize {
        let new_node_id = self.get_node_id();
        self.node_buffer
            .push(Node::new(new_node_id, NodeType::Lambda(var_id, expr)));
        self.node_buffer.len() - 1
    }

    fn variable_node(&mut self, var_id: u32) -> usize {
        let new_node_id = self.get_node_id();
        self.node_buffer
            .push(Node::new(new_node_id, NodeType::Variable(var_id)));
        self.node_buffer.len() - 1
    }
}

impl Index<usize> for NodeFactory {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        &self.node_buffer[index]
    }
}

impl IndexMut<usize> for NodeFactory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.node_buffer[index]
    }
}

// node 以下の変数名を unique に変更する
// 最初に呼ばれるだけのやつなので、lazy は含まれないと思ってよい。
pub fn alpha_convert(node_id: usize, parser_state: &mut ParserState, visited: &mut HashSet<u32>) {
    match parser_state.node_factory[node_id].node_type {
        NodeType::Boolean(_)
        | NodeType::Integer(_)
        | NodeType::String(_)
        | NodeType::Variable(_) => {}
        NodeType::Unary(_, child) => alpha_convert(child, parser_state, visited),
        NodeType::Binary(_, child1, child2) => {
            alpha_convert(child1, parser_state, visited);
            alpha_convert(child2, parser_state, visited);
        }
        NodeType::If(pred, first, second) => {
            alpha_convert(pred, parser_state, visited);
            alpha_convert(first, parser_state, visited);
            alpha_convert(second, parser_state, visited);
        }
        NodeType::Lambda(var_id, child) => {
            let new_var_id = parser_state.node_factory.get_var_id();
            // var_id を new_id に変更するための visited
            let mut local_visited = HashSet::new();
            replace_var_id(child, var_id, new_var_id, parser_state, &mut local_visited);
            parser_state.node_factory[node_id].node_type = NodeType::Lambda(new_var_id, child);

            alpha_convert(child, parser_state, visited);
        }
        NodeType::Lazy(var_id) => {
            if !visited.contains(&var_id) {
                visited.insert(var_id);
                let lazy_node_id = *parser_state.lazy_table.get(&var_id).unwrap();
                alpha_convert(lazy_node_id, parser_state, visited);
            }
        }
    }
}

fn replace_var_id(
    node_id: usize,
    from: u32,
    to: u32,
    parser_state: &mut ParserState,
    visited: &mut HashSet<u32>,
) {
    match parser_state.node_factory[node_id].node_type {
        NodeType::Boolean(_) | NodeType::Integer(_) | NodeType::String(_) => {}
        NodeType::Unary(_, child) => replace_var_id(child, from, to, parser_state, visited),
        NodeType::Binary(_, child1, child2) => {
            replace_var_id(child1, from, to, parser_state, visited);
            replace_var_id(child2, from, to, parser_state, visited);
        }
        NodeType::If(pred, first, second) => {
            replace_var_id(pred, from, to, parser_state, visited);
            replace_var_id(first, from, to, parser_state, visited);
            replace_var_id(second, from, to, parser_state, visited);
        }
        // Lambda の場合は、束縛変数と同じ名前の変数がある場合は置換しない
        NodeType::Lambda(var_id, child) => {
            if var_id != from {
                replace_var_id(child, from, to, parser_state, visited);
            }
        }
        NodeType::Variable(var_id) => {
            if var_id == from {
                parser_state.node_factory[node_id].node_type = NodeType::Variable(to);
            }
        }
        NodeType::Lazy(var_id) => {
            if !visited.contains(&var_id) {
                visited.insert(var_id);
                let cache_id = *parser_state.lazy_table.get(&var_id).unwrap();
                replace_var_id(cache_id, from, to, parser_state, visited);
            }
        }
    }
}

fn construct_node(
    parser_state: &mut ParserState,
    token_stream: &mut VecDeque<TokenType>,
) -> Result<usize, ParseError> {
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

pub fn print_node(parsre_state: &ParserState) {
    fn print_node_inner(parsre_state: &ParserState, node_id: usize, depth: usize) {
        let node = &parsre_state.node_factory[node_id];
        let indent = "    ".repeat(depth);
        match node.node_type.clone() {
            NodeType::Boolean(b) => {
                println!("{}Boolean({})", indent, b);
            }
            NodeType::Integer(i) => {
                println!("{}Integer({})", indent, i);
            }
            NodeType::String(s) => {
                println!("{}String({})", indent, s);
            }
            NodeType::Unary(opcode, child) => {
                println!("{}Unary({:?})", indent, opcode);
                print_node_inner(parsre_state, child, depth + 1);
            }
            NodeType::Binary(opcode, child1, child2) => {
                println!("{}Binary({:?})", indent, opcode);
                print_node_inner(parsre_state, child1, depth + 1);
                print_node_inner(parsre_state, child2, depth + 1);
            }
            NodeType::If(pred, first, second) => {
                println!("{}If", indent);
                print_node_inner(parsre_state, pred, depth + 1);
                print_node_inner(parsre_state, first, depth + 1);
                print_node_inner(parsre_state, second, depth + 1);
            }
            NodeType::Lambda(var_id, child) => {
                println!("{}Lambda({})", indent, var_id);
                print_node_inner(parsre_state, child, depth + 1);
            }
            NodeType::Variable(var_id) => {
                println!("{}Variable({})", indent, var_id);
            }
            NodeType::Lazy(var_id) => {
                println!("{}Lazy({})", indent, var_id);
            }
        }
    }
    print_node_inner(parsre_state, parsre_state.node_factory.root_id, 0);
    println!();
    println!("cache: ");
    for (k, v) in parsre_state.lazy_table.iter() {
        println!("key: {}", *k);
        print_node_inner(parsre_state, *v, 1);
    }
    println!("-----");
}

pub fn parse(input: String) -> Result<Node, ParseError> {
    let mut parser_state = ParserState::new();
    let token_list = tokenizer::tokenize(input)?;
    let mut queue = VecDeque::from_iter(token_list);
    let root_node_id = construct_node(&mut parser_state, &mut queue)?;
    parser_state.node_factory.root_id = root_node_id;

    let mut visited = HashSet::new();
    alpha_convert(
        parser_state.node_factory.root_id,
        &mut parser_state,
        &mut visited,
    );
    print_node(&parser_state);

    for iter in 0..14 {
        println!("iter: {}", iter);
        // for _iter in 0..10_000_000 {
        let mut updated = false;
        let root_id = parser_state.node_factory.root_id;
        evaluate_once(&mut parser_state, root_id, &mut updated);
        print_node(&parser_state);

        if !updated {
            break;
        }
    }
    let result = parser_state.node_factory[parser_state.node_factory.root_id].clone();
    Ok(result)
}

// apply をするために variable(var_id) を node で置換する
pub fn substitute(
    root_node_id: usize,
    var_id: u32,
    node_id: usize,
    parser_state: &mut ParserState,
) {
    // Variable(X) を Lazy(X) で置換する
    fn substitute_inner(
        node_id: usize,
        var_id: u32,
        parser_state: &mut ParserState,
        visited: &mut HashSet<usize>,
    ) {
        let nt = parser_state.node_factory[node_id].node_type.clone();
        match nt {
            NodeType::Boolean(_) | NodeType::Integer(_) | NodeType::String(_) => {}
            NodeType::Unary(_, child) => substitute_inner(child, var_id, parser_state, visited),
            NodeType::Binary(_, child1, child2) => {
                substitute_inner(child1, var_id, parser_state, visited);
                substitute_inner(child2, var_id, parser_state, visited);
            }
            NodeType::If(pred, first, second) => {
                substitute_inner(pred, var_id, parser_state, visited);
                substitute_inner(first, var_id, parser_state, visited);
                substitute_inner(second, var_id, parser_state, visited);
            }
            NodeType::Lambda(child_var_id, child) => {
                // 同名の束縛変数がある場合は置換しない
                if var_id != child_var_id {
                    substitute_inner(child, var_id, parser_state, visited);
                }
            }
            NodeType::Variable(child_var_id) => {
                if var_id == child_var_id {
                    parser_state.node_factory[node_id].node_type = NodeType::Lazy(var_id);
                }
            }
            NodeType::Lazy(child_var_id) => {
                if var_id == child_var_id {
                    // すでに置換済みの場合は何もしない
                    return;
                }
                let lazy_node_id = *parser_state.lazy_table.get(&child_var_id).unwrap();
                substitute_inner(lazy_node_id, var_id, parser_state, visited);
            }
        }
    }

    let mut visited = HashSet::new();
    parser_state.lazy_table.insert(var_id, node_id);
    substitute_inner(root_node_id, var_id, parser_state, &mut visited);
}

pub fn evaluate_once(parser_state: &mut ParserState, node_id: usize, updated: &mut bool) {
    match parser_state.node_factory[node_id].node_type {
        // 値の場合はそのまま返す
        NodeType::Boolean(_)
        | NodeType::Integer(_)
        | NodeType::String(_)
        | NodeType::Variable(_) => {}
        NodeType::Unary(opcode, child_id) => {
            let child_type = parser_state.node_factory[child_id].node_type.clone();

            match opcode {
                UnaryOpecode::Negate => match child_type {
                    NodeType::Integer(i) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Integer(-1 * i);
                    }
                    _ => {}
                },
                UnaryOpecode::Not => match child_type {
                    NodeType::Boolean(b) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Boolean(!b);
                    }
                    _ => {}
                },
                UnaryOpecode::StrToInt => match child_type {
                    NodeType::String(s) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type =
                            NodeType::Integer(s.to_i64());
                    }
                    _ => {}
                },
                UnaryOpecode::IntToStr => match child_type {
                    NodeType::Integer(i) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type =
                            NodeType::String(ICFPString::from_i64(i))
                    }
                    _ => {}
                },
            }
            if !*updated {
                evaluate_once(parser_state, child_id, updated);
            }
        }
        NodeType::Binary(opcode, child1, child2) => {
            let child_type1 = parser_state.node_factory[child1].node_type.clone();
            let child_type2 = parser_state.node_factory[child2].node_type.clone();
            match opcode {
                BinaryOpecode::Add => match (child_type1, child_type2) {
                    (NodeType::Integer(i1), NodeType::Integer(i2)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Integer(i1 + i2);
                    }
                    _ => {}
                },
                BinaryOpecode::Sub => match (child_type1, child_type2) {
                    (NodeType::Integer(i1), NodeType::Integer(i2)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Integer(i1 - i2);
                    }
                    _ => {}
                },
                BinaryOpecode::Mul => match (child_type1, child_type2) {
                    (NodeType::Integer(i1), NodeType::Integer(i2)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Integer(i1 * i2);
                    }
                    _ => {}
                },
                BinaryOpecode::Div => match (child_type1, child_type2) {
                    (NodeType::Integer(i1), NodeType::Integer(i2)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Integer(i1 / i2);
                    }
                    _ => {}
                },
                BinaryOpecode::Modulo => match (child_type1, child_type2) {
                    (NodeType::Integer(i1), NodeType::Integer(i2)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Integer(i1 % i2);
                    }
                    _ => {}
                },
                BinaryOpecode::IntegerLarger => match (child_type1, child_type2) {
                    (NodeType::Integer(i1), NodeType::Integer(i2)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Boolean(i1 < i2);
                    }
                    _ => {}
                },
                BinaryOpecode::IntegerSmaller => match (child_type1, child_type2) {
                    (NodeType::Integer(i1), NodeType::Integer(i2)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Boolean(i1 > i2);
                    }
                    _ => {}
                },
                BinaryOpecode::Equal => match (child_type1, child_type2) {
                    (NodeType::Integer(i1), NodeType::Integer(i2)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Boolean(i1 == i2);
                    }
                    (NodeType::String(s1), NodeType::String(s2)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Boolean(s1 == s2);
                    }
                    (NodeType::Boolean(b1), NodeType::Boolean(b2)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Boolean(b1 == b2);
                    }
                    _ => {}
                },
                BinaryOpecode::Or => match (child_type1, child_type2) {
                    (NodeType::Boolean(b1), NodeType::Boolean(b2)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Boolean(b1 || b2);
                    }
                    _ => {}
                },
                BinaryOpecode::And => match (child_type1, child_type2) {
                    (NodeType::Boolean(b1), NodeType::Boolean(b2)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Boolean(b1 && b2);
                    }
                    _ => {}
                },
                BinaryOpecode::StrConcat => match (child_type1, child_type2) {
                    (NodeType::String(s1), NodeType::String(s2)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type =
                            NodeType::String(s1.concat(&s2));
                    }
                    _ => {}
                },
                BinaryOpecode::TakeStr => match (child_type1, child_type2) {
                    (NodeType::Integer(i), NodeType::String(s)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type =
                            NodeType::String(s.take(i as usize));
                    }
                    _ => {}
                },
                BinaryOpecode::DropStr => match (child_type1, child_type2) {
                    (NodeType::Integer(i), NodeType::String(s)) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type =
                            NodeType::String(s.drop(i as usize));
                    }
                    _ => {}
                },
                BinaryOpecode::Apply => match child_type1 {
                    NodeType::Lambda(var_id, child1_inner) => {
                        *updated = true;
                        substitute(child1_inner, var_id, child2, parser_state);
                        parser_state.node_factory[node_id].node_type =
                            parser_state.node_factory[child1_inner].node_type.clone();
                    }
                    NodeType::Lazy(var_id) => {
                        // lazy の中身がある場合は、そこも見る
                        let lazy_node_id = *parser_state.lazy_table.get(&var_id).unwrap();
                        let lazy_node_type =
                            parser_state.node_factory[lazy_node_id].node_type.clone();
                        match lazy_node_type {
                            NodeType::Lambda(lazy_var_id, lazy_child1_inner) => {
                                *updated = true;
                                substitute(lazy_child1_inner, lazy_var_id, child2, parser_state);
                                parser_state.node_factory[lazy_node_id].node_type = parser_state
                                    .node_factory[lazy_child1_inner]
                                    .node_type
                                    .clone();
                                // Apply の第1項が lambda の時、lambda の中身を substitute して更新するだけではなく、
                                // Apply を適用した結果 lazy で上書きする必要がある
                                parser_state.node_factory[node_id].node_type =
                                    NodeType::Lazy(var_id);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                },
            }
            if !*updated {
                evaluate_once(parser_state, child2, updated);
                if !*updated {
                    evaluate_once(parser_state, child1, updated);
                }
            }
        }
        NodeType::If(pred, first, second) => match parser_state.node_factory[pred].node_type {
            NodeType::Boolean(b) => {
                if b {
                    *updated = true;
                    parser_state.node_factory[node_id].node_type =
                        parser_state.node_factory[first].node_type.clone();
                } else {
                    *updated = true;
                    parser_state.node_factory[node_id].node_type =
                        parser_state.node_factory[second].node_type.clone();
                }
            }
            _ => {
                if !*updated {
                    evaluate_once(parser_state, pred, updated);
                    if !*updated {
                        evaluate_once(parser_state, first, updated);
                        if !*updated {
                            evaluate_once(parser_state, second, updated);
                        }
                    }
                }
            }
        },
        NodeType::Lambda(var_id, child) => {
            if !*updated {
                evaluate_once(parser_state, child, updated);
            }
        }
        NodeType::Lazy(var_id) => {
            if let Some(lazy_node) = parser_state.lazy_table.get(&var_id) {
                let lazy_node = *lazy_node;
                // プリミティブ型に縮約された場合は、Lazy ノードを置換する
                match parser_state.node_factory[lazy_node].node_type {
                    NodeType::Boolean(_)
                    | NodeType::Integer(_)
                    | NodeType::String(_)
                    | NodeType::Variable(_) => {
                        *updated = true;
                        parser_state.node_factory[node_id].node_type =
                            parser_state.node_factory[lazy_node].node_type.clone();
                    }
                    NodeType::Lazy(next_var_id) => {
                        // Lazy(x) -> Lazy(y) -> N のように Lazy 続きの場合は、 Lazy (x) -> N に縮約する
                        *updated = true;
                        parser_state.node_factory[node_id].node_type = NodeType::Lazy(next_var_id);
                    }
                    _ => {
                        if !*updated {
                            evaluate_once(parser_state, lazy_node, updated);
                        }
                    }
                }
            } else {
                unreachable!("Lazy node should be replaced by its value")
            }
        }
    }
}

pub struct ParserState {
    node_factory: NodeFactory,
    lazy_table: HashMap<u32, usize>,
}

impl ParserState {
    pub fn new() -> ParserState {
        ParserState {
            node_factory: NodeFactory::new(),
            lazy_table: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_sequence(input: &str, expected: NodeType) {
        let result = parse(input.to_string()).unwrap();
        assert_eq!(result.node_type, expected);
    }

    // testcase is generated from https://icfpcontest2024.github.io/icfp.html

    #[test]
    fn test_unary_negate() {
        test_sequence("U- I$", NodeType::Integer(-3));
    }

    #[test]
    fn test_unary_not() {
        test_sequence("U! B= S$ S$", NodeType::Boolean(false));
        test_sequence("U! B= I/ I$", NodeType::Boolean(true));
    }

    #[test]
    fn test_unary_strtoint() {
        test_sequence("U# S4%34", NodeType::Integer(15818151));
    }

    #[test]
    fn test_add() {
        test_sequence("B+ I# I$", NodeType::Integer(5));
    }

    #[test]
    fn test_sub() {
        test_sequence("B- I$ I#", NodeType::Integer(1));
    }

    #[test]
    fn test_mul() {
        test_sequence("B* I# I$", NodeType::Integer(6));
    }

    #[test]
    fn test_div() {
        test_sequence("B/ U- I( I#", NodeType::Integer(-3));
    }

    #[test]
    fn test_mod() {
        test_sequence("B% U- I( I#", NodeType::Integer(-1));
    }

    #[test]
    fn test_gt() {
        test_sequence("B< I$ I#", NodeType::Boolean(false));
        test_sequence("B< I# I$", NodeType::Boolean(true));
    }

    #[test]
    fn test_lt() {
        test_sequence("B> I$ I#", NodeType::Boolean(true));
        test_sequence("B> I# I$", NodeType::Boolean(false));
    }

    #[test]
    fn test_eq() {
        test_sequence("B= I$ I#", NodeType::Boolean(false));
        test_sequence("B= I$ B+ I# I\"", NodeType::Boolean(true));

        test_sequence("B= S# S#", NodeType::Boolean(true));
        test_sequence("B= S# S$", NodeType::Boolean(false));

        test_sequence("B= T B= F F", NodeType::Boolean(true));
        test_sequence("B= F B= F F", NodeType::Boolean(false));
    }

    #[test]
    fn test_and() {
        test_sequence("B& T F", NodeType::Boolean(false));
        test_sequence("B& T T", NodeType::Boolean(true));
    }

    #[test]
    fn test_or() {
        test_sequence("B| T F", NodeType::Boolean(true));
        test_sequence("B| F F", NodeType::Boolean(false));
    }

    #[test]
    fn test_concat() {
        let expected = ICFPString::from_rawstr("#$").unwrap();
        test_sequence("B. S# S$", NodeType::String(expected));
    }

    #[test]
    fn test_take() {
        let expected = ICFPString::from_rawstr("#a").unwrap();
        test_sequence("BT I# S#agc4gs", NodeType::String(expected));
    }

    #[test]
    fn test_drop() {
        let expected = ICFPString::from_rawstr("gc4gs").unwrap();
        test_sequence("BD I# S#agc4gs", NodeType::String(expected));
    }

    #[test]
    fn test_if() {
        test_sequence("? T I# I$", NodeType::Integer(2));
        test_sequence("? F I# I$", NodeType::Integer(3));
        test_sequence(
            "? B> I# I$ S9%3 S./",
            NodeType::String(ICFPString::from_rawstr("./").unwrap()),
        );
    }

    #[test]
    fn test_lambda_apply1() {
        test_sequence("B$ L# B$ L\" B+ v\" v\" B* I$ I# v8", NodeType::Integer(12));
    }

    #[test]
    fn test_lambda_apply2() {
        test_sequence(
            "B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK",
            NodeType::String(ICFPString::from_rawstr("B%,,/}Q/2,$_").unwrap()),
        )
    }

    #[test]
    fn test_lambda_apply4() {
        test_sequence(
                    "B$ B$ L\" B$ L# B$ v\" B$ v# v# L# B$ v\" B$ v# v# L\" L# ? B= v# I! I\" B$ L$ B+ B$ v\" v$ B$ v\" v$ B- v# I\" I%",
                    NodeType::Integer(16),
                )
    }
}
