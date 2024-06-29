use super::icfpstring::ICFPString;
use super::ParseError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnaryOpecode {
    Negate,
    Not,
    StrToInt,
    IntToStr,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryOpecode {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    IntegerLarger,
    IntegerSmaller,
    Equal,
    Or,
    And,
    StrConcat,
    TakeStr,
    DropStr,
    Apply,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Boolean(bool),
    Integer(i64),
    String(ICFPString),
    Unary(UnaryOpecode),
    Binary(BinaryOpecode),
    If,
    Lambda(i64),
    Variable(i64),
}

pub fn tokenize(input: String) -> Result<Vec<TokenType>, ParseError> {
    let mut ret = vec![];
    for token_str in input.split_ascii_whitespace() {
        let chars = token_str.chars().collect::<Vec<_>>();
        match chars[0] {
            'T' => ret.push(TokenType::Boolean(true)),
            'F' => ret.push(TokenType::Boolean(false)),
            'I' => {
                let s = ICFPString::from_str(chars[1..].to_vec())?;
                let num = s.to_i64();
                ret.push(TokenType::Integer(num));
            }
            'S' => {
                let s = ICFPString::from_str(chars[1..].to_vec())?;
                ret.push(TokenType::String(s));
            }
            'U' => match chars[1] {
                '-' => ret.push(TokenType::Unary(UnaryOpecode::Negate)),
                '!' => ret.push(TokenType::Unary(UnaryOpecode::Not)),
                '#' => ret.push(TokenType::Unary(UnaryOpecode::StrToInt)),
                '$' => ret.push(TokenType::Unary(UnaryOpecode::IntToStr)),
                _ => return Err(ParseError::InvalidToken),
            },
            'B' => match chars[1] {
                '+' => ret.push(TokenType::Binary(BinaryOpecode::Add)),
                '-' => ret.push(TokenType::Binary(BinaryOpecode::Sub)),
                '*' => ret.push(TokenType::Binary(BinaryOpecode::Mul)),
                '/' => ret.push(TokenType::Binary(BinaryOpecode::Div)),
                '%' => ret.push(TokenType::Binary(BinaryOpecode::Modulo)),
                '<' => ret.push(TokenType::Binary(BinaryOpecode::IntegerLarger)),
                '>' => ret.push(TokenType::Binary(BinaryOpecode::IntegerSmaller)),
                '=' => ret.push(TokenType::Binary(BinaryOpecode::Equal)),
                '|' => ret.push(TokenType::Binary(BinaryOpecode::Or)),
                '&' => ret.push(TokenType::Binary(BinaryOpecode::And)),
                '.' => ret.push(TokenType::Binary(BinaryOpecode::StrConcat)),
                'T' => ret.push(TokenType::Binary(BinaryOpecode::TakeStr)),
                'D' => ret.push(TokenType::Binary(BinaryOpecode::DropStr)),
                '$' => ret.push(TokenType::Binary(BinaryOpecode::Apply)),
                _ => return Err(ParseError::InvalidToken),
            },
            '?' => ret.push(TokenType::If),
            'L' => {
                let s = ICFPString::from_str(chars[1..].to_vec())?;
                let num = s.to_i64();
                ret.push(TokenType::Lambda(num));
            }
            'v' => {
                let s = ICFPString::from_str(chars[1..].to_vec())?;
                let num = s.to_i64();
                ret.push(TokenType::Variable(num));
            }
            _ => return Err(ParseError::InvalidToken),
        }
    }
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use crate::parser::{icfpstring::ICFPString, tokenizer::TokenType};

    use super::{tokenize, BinaryOpecode, UnaryOpecode};

    fn run_single_token_test(s: &str, expected: TokenType) {
        let input = s.to_string();
        let token_list = tokenize(input).unwrap();
        assert_eq!(token_list.len(), 1);

        assert_eq!(token_list[0], expected);
    }

    #[test]
    fn test_example_true() {
        run_single_token_test("T", TokenType::Boolean(true));
    }

    #[test]
    fn test_example_false() {
        run_single_token_test("F", TokenType::Boolean(false));
    }

    #[test]
    fn test_example_integer() {
        run_single_token_test("I/6", TokenType::Integer(1337));
    }

    #[test]
    fn test_example_unary_neg() {
        run_single_token_test("U-", TokenType::Unary(UnaryOpecode::Negate));
    }

    #[test]
    fn test_example_unary_not() {
        run_single_token_test("U!", TokenType::Unary(UnaryOpecode::Not));
    }

    #[test]
    fn test_example_unary_stoi() {
        run_single_token_test("U#", TokenType::Unary(UnaryOpecode::StrToInt));
    }

    #[test]
    fn test_example_unary_itos() {
        run_single_token_test("U$", TokenType::Unary(UnaryOpecode::IntToStr));
    }

    #[test]
    fn test_example_binary_add() {
        run_single_token_test("B+", TokenType::Binary(BinaryOpecode::Add));
    }

    #[test]
    fn test_example_binary_sub() {
        run_single_token_test("B-", TokenType::Binary(BinaryOpecode::Sub));
    }

    #[test]
    fn test_example_binary_mul() {
        run_single_token_test("B*", TokenType::Binary(BinaryOpecode::Mul));
    }

    #[test]
    fn test_example_binary_div() {
        run_single_token_test("B/", TokenType::Binary(BinaryOpecode::Div));
    }

    #[test]
    fn test_example_binary_mod() {
        run_single_token_test("B%", TokenType::Binary(BinaryOpecode::Modulo));
    }

    #[test]
    fn test_example_binary_int_larger() {
        run_single_token_test("B<", TokenType::Binary(BinaryOpecode::IntegerLarger));
    }

    #[test]
    fn test_example_binary_int_smaller() {
        run_single_token_test("B>", TokenType::Binary(BinaryOpecode::IntegerSmaller));
    }

    #[test]
    fn test_example_binary_equal() {
        run_single_token_test("B=", TokenType::Binary(BinaryOpecode::Equal));
    }

    #[test]
    fn test_example_binary_or() {
        run_single_token_test("B|", TokenType::Binary(BinaryOpecode::Or));
    }

    #[test]
    fn test_example_binary_and() {
        run_single_token_test("B&", TokenType::Binary(BinaryOpecode::And));
    }

    #[test]
    fn test_example_binary_str_concat() {
        run_single_token_test("B.", TokenType::Binary(BinaryOpecode::StrConcat));
    }

    #[test]
    fn test_example_binary_take_str() {
        run_single_token_test("BT", TokenType::Binary(BinaryOpecode::TakeStr));
    }

    #[test]
    fn test_example_binary_drop_str() {
        run_single_token_test("BD", TokenType::Binary(BinaryOpecode::DropStr));
    }

    #[test]
    fn test_example_binary_apply() {
        run_single_token_test("B$", TokenType::Binary(BinaryOpecode::Apply));
    }

    #[test]
    fn test_example_if() {
        run_single_token_test("?", TokenType::If);
    }

    #[test]
    fn test_example_lambda() {
        run_single_token_test("L/6", TokenType::Lambda(1337));
    }

    #[test]
    fn test_example_variable() {
        run_single_token_test("v/6", TokenType::Variable(1337));
    }

    #[test]
    fn test_multiple_token() {
        let input = "? B> I# I$ S9%3 S./";
        let token_list = tokenize(input.to_string()).unwrap();
        assert_eq!(token_list.len(), 6);
        let expected = vec![
            TokenType::If,
            TokenType::Binary(BinaryOpecode::IntegerSmaller),
            TokenType::Integer(2),
            TokenType::Integer(3),
            TokenType::String(ICFPString::from_str("9%3".chars().collect()).unwrap()),
            TokenType::String(ICFPString::from_str("./".chars().collect()).unwrap()),
        ];

        assert_eq!(token_list, expected);
    }

    #[test]
    fn test_invalid_token() {
        let input = "X";
        let result = tokenize(input.to_string());
        assert!(result.is_err());
    }
}
