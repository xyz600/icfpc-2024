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

fn tokenize(input: String) -> Result<Vec<TokenType>, ParseError> {
    let mut ret = vec![];
    for token_str in input.split_ascii_whitespace() {
        let chars = token_str.chars().collect::<Vec<_>>();
        match chars[0] {
            'T' => ret.push(TokenType::BooleanTrue),
            'F' => ret.push(TokenType::BooleanFalse),
            'I' => {
                // TODO: FIXME
                let num = token_str[1..].parse::<i64>().unwrap();
                ret.push(TokenType::Integer(num));
            }
            'S' => {
                // TODO: FIXME
                let str = token_str[1..].to_string();
                ret.push(TokenType::String(str));
            }
            'U' => match chars[1] {
                '-' => ret.push(TokenType::UnaryNegate),
                '!' => ret.push(TokenType::UnaryNot),
                '#' => ret.push(TokenType::UnaryStrToInt),
                '$' => ret.push(TokenType::UnaryIntToStr),
                _ => return Err(ParseError::InvalidToken),
            },
            'B' => match chars[1] {
                '+' => ret.push(TokenType::BinaryAdd),
                '-' => ret.push(TokenType::BinarySub),
                '*' => ret.push(TokenType::BinaryMul),
                '/' => ret.push(TokenType::BinaryDiv),
                '%' => ret.push(TokenType::BinaryModulo),
                '<' => ret.push(TokenType::BinaryIntegerLarger),
                '>' => ret.push(TokenType::BinaryIntegerSmaller),
                '=' => ret.push(TokenType::BinaryEqual),
                '|' => ret.push(TokenType::BinaryOr),
                '&' => ret.push(TokenType::BinaryAnd),
                '.' => ret.push(TokenType::BinaryStrConcat),
                'T' => ret.push(TokenType::BinaryTakeStr),
                'D' => ret.push(TokenType::BinaryDropStr),
                '$' => ret.push(TokenType::BinaryApply),
                _ => return Err(ParseError::InvalidToken),
            },
            '?' => ret.push(TokenType::If),
            'L' => {
                // TODO: FIXME
                let num = token_str[1..].parse::<i64>().unwrap();
                ret.push(TokenType::Lambda(num));
            }
            _ => return Err(ParseError::InvalidToken),
        }
    }
    Ok(ret)
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
