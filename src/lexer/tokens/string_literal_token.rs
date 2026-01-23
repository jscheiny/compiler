use std::fmt::Display;

use crate::lexer::Token;

#[derive(Clone)]
pub struct StringLiteralToken(pub String);

impl Display for StringLiteralToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

const DOUBLE_QUOTE: char = '"';
const ESCAPE: char = '\\';

pub fn try_tokenize_string_literal(text: &str) -> Option<(Token, usize)> {
    if !text.starts_with(DOUBLE_QUOTE) {
        return None;
    }

    let mut skip_endquote = false;
    let mut has_endquote = false;
    let mut count = 0;

    for character in text[1..].chars() {
        if character == DOUBLE_QUOTE && !skip_endquote {
            has_endquote = true;
            break;
        }

        if character == '\n' {
            return None;
        }

        skip_endquote = character == ESCAPE;
        count += character.len_utf8();
    }

    if !has_endquote {
        return None;
    }

    let string = text[1..=count].to_string();
    let token = Token::StringLiteral(StringLiteralToken(string));
    Some((token, count + 2))
}
