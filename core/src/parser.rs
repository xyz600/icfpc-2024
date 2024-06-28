use std::fmt::Display;

pub enum TokenType {
    BooleanTrue,
    BooleanFalse,
    Integer(i64),
    String(String),
    UnaryNegate,
    UnaryNot,
    UnaryStrToInt,
    UnaryIntToStr,
    BinaryAdd,
    BinarySub,
    BinaryMul,
    BinaryDiv,
    BinaryModulo,
    BinaryIntegerLarger,
    BinaryIntegerSmaller,
    BinaryEqual,
    BinaryOr,
    BinaryAnd,
    BinaryStrConcat,
    BinaryTakeStr,
    BinaryDropStr,
    BinaryApply,
    If,
    Lambda(i64),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    InvalidCharacter(i64),
    InvalidToken,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidCharacter(i64) => write!(f, "Invalid character {}", i64),
            ParseError::InvalidToken => write!(f, "Invalid token"),
        }
    }
}

const CHAR_MAP: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";
const START_CH: char = '!';

fn to_vec_char(s: &str) -> Vec<char> {
    s.chars().collect::<Vec<_>>()
}

fn rawstr_to_str(input: Vec<char>) -> Result<Vec<char>, ParseError> {
    let char_map = to_vec_char(CHAR_MAP);

    let mut ret = vec![];
    for ch in input.iter() {
        let index = *ch as i64 - START_CH as i64;
        if index < 0 || index >= CHAR_MAP.len() as i64 {
            return Err(ParseError::InvalidCharacter(*ch as i64));
        }
        ret.push(char_map[index as usize]);
    }
    Ok(ret)
}

fn str_to_rawstr(input: Vec<char>) -> Result<Vec<char>, ParseError> {
    let char_map = to_vec_char(CHAR_MAP);
    let mut ret = vec![];
    for ch in input.iter() {
        let index = char_map.iter().position(|&x| x == *ch);
        if let Some(index) = index {
            let new_index = index as i64 + START_CH as i64;
            let new_char = std::char::from_u32(new_index as u32);
            if let Some(ch) = new_char {
                ret.push(ch);
            } else {
                return Err(ParseError::InvalidCharacter(new_index));
            }
        } else {
            return Err(ParseError::InvalidCharacter(*ch as i64));
        }
    }
    Ok(ret)
}

#[cfg(test)]
mod tests_rawstr {
    use super::*;

    #[test]
    fn test_rawstr_to_str() {
        let mut input = vec![];
        for i in 0..CHAR_MAP.len() {
            let ch = std::char::from_u32(START_CH as u32 + i as u32).unwrap();
            input.push(ch);
        }
        let input = input;
        let output = rawstr_to_str(input).unwrap();

        for (i, &ch) in output.iter().enumerate() {
            assert_eq!(ch, CHAR_MAP.chars().nth(i).unwrap());
        }
    }

    #[test]
    fn test_rawstr_to_str_invalid_char() {
        let input = vec![' '];
        let output = rawstr_to_str(input);
        assert!(output.is_err());
    }

    #[test]
    fn test_str_to_rawstr1() {
        let input = to_vec_char(CHAR_MAP);
        let output = str_to_rawstr(input).unwrap();

        for (i, &ch) in output.iter().enumerate() {
            let expected_char_index = START_CH as i64 + i as i64;
            let char = std::char::from_u32(expected_char_index as u32).unwrap();
            assert_eq!(ch, char);
        }
    }

    #[test]
    fn test_str_to_rawstr2() {
        let input = vec![std::char::from_u32(22).unwrap()];
        let output = str_to_rawstr(input);
        assert!(output.is_err());
    }
}

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
