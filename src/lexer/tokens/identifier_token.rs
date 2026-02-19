use crate::lexer::{Token, TokenWidth, TryTokenizeResult};

pub fn try_tokenize_identifier(text: &str) -> Option<TryTokenizeResult> {
    let mut width = TokenWidth::new();
    let mut identifier = String::from("");
    for character in text.chars() {
        if !character.is_alphanumeric() && character != '_' {
            break;
        }

        identifier.push(character);
        width.add_char(character);
    }

    let len = identifier.len();
    if len == 0 {
        return None;
    }

    Some(TryTokenizeResult {
        token: Some(Token::Identifier(identifier)),
        width,
    })
}
