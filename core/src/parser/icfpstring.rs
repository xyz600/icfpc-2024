use std::{fmt::Display, ops::Index};

use macro_util::str_to_char_array;

use super::ParseError;

const CHAR_MAP: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";
str_to_char_array!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n");

const START_CH: char = '!';

/// ICFP の中で使われる文字列 ("S..." や "I..." など)の表現
/// 標準文字列に修正したり、base-94 文字列の数値変換が行いやすいようにする
#[derive(Debug, Clone)]
pub struct ICFPString {
    s: Vec<u8>,
}

impl ICFPString {
    pub fn new(s: Vec<u8>) -> ICFPString {
        ICFPString { s }
    }

    pub fn from_str(input: Vec<char>) -> Result<ICFPString, ParseError> {
        let mut s = vec![];
        for ch in input.iter() {
            let index = *ch as i64 - START_CH as i64;
            if index < 0 || index >= CHAR_MAP.len() as i64 {
                return Err(ParseError::InvalidCharacter(*ch as i64));
            }
            s.push(index as u8);
        }
        Ok(ICFPString { s })
    }

    pub fn from_i64(input: i64) -> ICFPString {
        let mut s = vec![];
        let mut input = input;
        while input > 0 {
            s.push((input % 94) as u8);
            input /= 94;
        }
        s.reverse();
        ICFPString { s }
    }

    pub fn to_string(&self) -> Result<Vec<char>, ParseError> {
        let mut ret = vec![];
        for index in self.s.iter() {
            let new_index = *index as u32 + START_CH as u32;
            let ch = std::char::from_u32(new_index)
                .ok_or(ParseError::InvalidCharacter(new_index as i64))?;
            ret.push(ch);
        }
        Ok(ret)
    }

    pub fn to_i64(&self) -> i64 {
        let mut ret = 0;
        for index in self.s.iter() {
            ret = ret * 94 + *index as i64;
        }
        ret
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &char> {
        self.s.iter().map(|&index| &ARRAY[index as usize])
    }

    pub fn concat(&self, other: &ICFPString) -> ICFPString {
        let mut s = self.s.clone();
        s.extend(other.s.iter());
        ICFPString { s }
    }

    pub fn take(&self, n: usize) -> ICFPString {
        let s = self.s.iter().take(n).copied().collect();
        ICFPString { s }
    }

    pub fn drop(&self, n: usize) -> ICFPString {
        let s = self.s.iter().skip(n).copied().collect();
        ICFPString { s }
    }
}

impl PartialEq for ICFPString {
    fn eq(&self, other: &Self) -> bool {
        self.s == other.s
    }
}

impl Index<usize> for ICFPString {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        let char_index = self.s[index] as usize;
        &ARRAY[char_index]
    }
}

impl Display for ICFPString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let char_map = to_vec_char(CHAR_MAP);
        let mut char_buffer = vec![];
        for index in self.s.iter() {
            let ch = char_map.get(*index as usize).ok_or(std::fmt::Error)?;
            char_buffer.push(*ch);
        }
        let s = char_buffer.iter().collect::<String>();
        write!(f, "{}", s)?;

        Ok(())
    }
}

fn to_vec_char(s: &str) -> Vec<char> {
    s.chars().collect::<Vec<_>>()
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
        let output = ICFPString::from_str(input.clone()).unwrap();

        for (i, &ch) in output.iter().enumerate() {
            assert_eq!(ch, CHAR_MAP.chars().nth(i).unwrap());
        }

        let raw_str = output.to_string().unwrap();
        assert_eq!(raw_str, input);
    }

    #[test]
    fn test_rawstr_to_str_invalid_char() {
        let input = vec![' '];
        let output = ICFPString::from_str(input);
        assert!(output.is_err());
    }

    #[test]
    fn test_example1() {
        let input = to_vec_char("!");
        let output = ICFPString::from_str(input).unwrap();
        let expected = to_vec_char("a");
        assert_eq!(output[0], expected[0]);
    }

    #[test]
    fn test_example2() {
        let input = to_vec_char("B");
        let output = ICFPString::from_str(input).unwrap();
        let expected = to_vec_char("H");
        assert_eq!(output[0], expected[0]);
    }

    #[test]
    fn test_example3() {
        let input = to_vec_char("!");
        let output = ICFPString::from_str(input).unwrap();
        let expected = to_vec_char("a");
        for (i, &ch) in output.iter().enumerate() {
            assert_eq!(ch, expected[i]);
        }
    }

    #[test]
    fn test_example() {
        let input = to_vec_char("B%,,/}Q/2,$_");
        let output = ICFPString::from_str(input).unwrap();
        let expected = to_vec_char("Hello World!");

        for (i, &ch) in output.iter().enumerate() {
            assert_eq!(ch, expected[i]);
        }
    }

    #[test]
    fn test_fromi64() {
        let input = 1337;
        let output = ICFPString::from_i64(input).to_string().unwrap();
        let expected = to_vec_char("/6");

        assert_eq!(output, expected);
    }

    #[test]
    fn test_toi64() {
        let input = to_vec_char("/6");
        let s = ICFPString::from_str(input).unwrap();
        let output = s.to_i64();
        let expected = 1337;
        assert_eq!(output, expected);
    }
}
