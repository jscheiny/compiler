use std::fmt::Display;

use crate::lexer::Token;

#[derive(Clone)]
pub struct IdentifierToken(pub String);

impl Display for IdentifierToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn try_tokenize_identifier(text: &str) -> Option<(Token, usize)> {
    let mut identifier = String::from("");
    for character in text.chars() {
        if identifier.is_empty() && !character.is_alphabetic() {
            return None;
        }

        if !character.is_alphanumeric() && character != '_' {
            break;
        }

        identifier.push(character);
    }

    let len = identifier.len();
    if len == 0 {
        None
    } else {
        Some((Token::Identifier(IdentifierToken(identifier)), len))
    }
}
