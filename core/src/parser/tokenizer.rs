use super::icfpstring::ICFPString;
use super::ParseError;

#[derive(Debug, Clone)]
pub enum TokenType {
    Boolean(bool),
    Integer(i64),
    String(ICFPString),
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
    Variable(i64),
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Lambda(l0), Self::Lambda(r0)) => l0 == r0,
            (Self::Variable(l0), Self::Variable(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
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

    use super::tokenize;

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
        run_single_token_test("U-", TokenType::UnaryNegate);
    }

    #[test]
    fn test_example_unary_not() {
        run_single_token_test("U!", TokenType::UnaryNot);
    }

    #[test]
    fn test_example_unary_stoi() {
        run_single_token_test("U#", TokenType::UnaryStrToInt);
    }

    #[test]
    fn test_example_unary_itos() {
        run_single_token_test("U$", TokenType::UnaryIntToStr);
    }

    #[test]
    fn test_example_binary_add() {
        run_single_token_test("B+", TokenType::BinaryAdd);
    }

    #[test]
    fn test_example_binary_sub() {
        run_single_token_test("B-", TokenType::BinarySub);
    }

    #[test]
    fn test_example_binary_mul() {
        run_single_token_test("B*", TokenType::BinaryMul);
    }

    #[test]
    fn test_example_binary_div() {
        run_single_token_test("B/", TokenType::BinaryDiv);
    }

    #[test]
    fn test_example_binary_mod() {
        run_single_token_test("B%", TokenType::BinaryModulo);
    }

    #[test]
    fn test_example_binary_int_larger() {
        run_single_token_test("B<", TokenType::BinaryIntegerLarger);
    }

    #[test]
    fn test_example_binary_int_smaller() {
        run_single_token_test("B>", TokenType::BinaryIntegerSmaller);
    }

    #[test]
    fn test_example_binary_equal() {
        run_single_token_test("B=", TokenType::BinaryEqual);
    }

    #[test]
    fn test_example_binary_or() {
        run_single_token_test("B|", TokenType::BinaryOr);
    }

    #[test]
    fn test_example_binary_and() {
        run_single_token_test("B&", TokenType::BinaryAnd);
    }

    #[test]
    fn test_example_binary_str_concat() {
        run_single_token_test("B.", TokenType::BinaryStrConcat);
    }

    #[test]
    fn test_example_binary_take_str() {
        run_single_token_test("BT", TokenType::BinaryTakeStr);
    }

    #[test]
    fn test_example_binary_drop_str() {
        run_single_token_test("BD", TokenType::BinaryDropStr);
    }

    #[test]
    fn test_example_binary_apply() {
        run_single_token_test("B$", TokenType::BinaryApply);
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
            TokenType::BinaryIntegerSmaller,
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
