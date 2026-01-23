use crate::lexer::{IgnoredToken, Token};

#[derive(Debug, Clone)]
pub struct WhitespaceToken {
    pub new_lines: usize,
    pub columns_since_last_new_line: usize,
}

pub fn try_tokenize_whitespace(text: &str) -> Option<(Token, usize)> {
    let mut bytes = 0;
    let mut token = IgnoredToken::new();
    for character in text.chars() {
        if !character.is_whitespace() {
            break;
        }

        bytes += character.len_utf8();
        token.add(character);
    }

    if bytes == 0 {
        return None;
    }

    Some((Token::Ignored(token), bytes))
}
