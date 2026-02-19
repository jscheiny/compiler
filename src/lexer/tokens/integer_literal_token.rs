use crate::lexer::{Token, TokenWidth, TryTokenizeResult};

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
        token: Some(Token::IntegerLiteral(*value)),
        width,
    })
}
