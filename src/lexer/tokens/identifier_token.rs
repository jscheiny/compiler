use std::fmt::Display;

use crate::lexer::{Token, TokenWidth, TryTokenizeResult};

#[derive(Clone)]
pub struct IdentifierToken(pub String);

impl Display for IdentifierToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn try_tokenize_identifier(text: &str) -> Option<TryTokenizeResult> {
    let mut width = TokenWidth::new();
    let mut identifier = String::from("");
    for character in text.chars() {
        if identifier.is_empty() && !character.is_alphabetic() && character != '_' {
            return None;
        }

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
        token: Some(Token::Identifier(IdentifierToken(identifier))),
        width,
    })
}
