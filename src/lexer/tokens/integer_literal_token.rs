use std::fmt::Display;

use crate::lexer::{Token, TokenWidth, TryTokenizeResult};

#[derive(Clone)]
pub struct IntegerLiteralToken(pub i64);

impl Display for IntegerLiteralToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn try_tokenize_integer_literal(text: &str) -> Option<TryTokenizeResult> {
    let mut width = TokenWidth::new();
    for character in text.chars() {
        if width.bytes == 0 && !character.is_numeric() {
            return None;
        }

        if !character.is_numeric() {
            break;
        }

        width.add_char(character);
    }

    let maybe_value = &text[0..width.bytes].parse::<i64>().ok();
    maybe_value.as_ref().map(|value| TryTokenizeResult {
        token: Some(Token::IntegerLiteral(IntegerLiteralToken(*value))),
        width,
    })
}
