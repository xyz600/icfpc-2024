use std::{fmt::Display, ops::Index};

fn str_to_int(s: String) -> Result<i64, ParseError> {
    let base: i64 = CHAR_MAP.bytes().len() as i64;
    assert_eq!(base, 94);

    let mut ret = 0i64;
    let chars = s.chars().collect::<Vec<_>>();
    for &ch in chars.iter() {
        let index = ch as i64 - START_CH as i64;
        if index < 0 || index >= base {
            return Err(ParseError::InvalidCharacter(ch as i64));
        }
        ret *= base;
        ret += index;
    }
    Ok(ret)
}

fn int_to_str(mut n: i64) -> Vec<char> {
    vec![]
}

pub enum Node {
    Boolean(bool),
    Integer(i64),
    String(String),
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
}

fn evaluate(mut node: Node) -> Node {
    node
}
