use crate::lexer::{IgnoredToken, Token, TokenWidth, TryTokenizeResult};

#[derive(Debug, Clone)]
pub struct WhitespaceToken {
    pub new_lines: usize,
    pub columns_since_last_new_line: usize,
}

pub fn try_tokenize_whitespace(text: &str) -> Option<TryTokenizeResult> {
    let mut width = TokenWidth::new();
    let mut token = IgnoredToken::new();
    for character in text.chars() {
        if !character.is_whitespace() {
            break;
        }

        width.add_char(character);
        token.add(character);
    }

    if width.bytes == 0 {
        return None;
    }

    Some(TryTokenizeResult {
        token: Token::Ignored(token),
        width,
    })
}
