use std::fmt::Display;

use crate::lexer::{Token, TokenParse};

#[derive(Clone)]
pub struct IntegerLiteralToken(pub i64);

impl TokenParse for IntegerLiteralToken {
    fn try_tokenize(text: &str) -> Option<(Token, usize)> {
        let mut count = 0;
        for character in text.chars() {
            if count == 0 && !character.is_numeric() {
                return None;
            }

            if !character.is_numeric() {
                break;
            }

            count += character.len_utf8();
        }

        let maybe_value = &text[0..count].parse::<i64>().ok();
        maybe_value
            .as_ref()
            .map(|value| (Token::IntegerLiteral(IntegerLiteralToken(*value)), count))
    }
}

impl Display for IntegerLiteralToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
