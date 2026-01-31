use crate::lexer::{TokenWidth, TryTokenizeResult};

pub fn try_tokenize_whitespace(text: &str) -> Option<TryTokenizeResult> {
    let mut width = TokenWidth::new();
    for character in text.chars() {
        if !character.is_whitespace() {
            break;
        }
        width.add_char(character);
    }

    if width.bytes == 0 {
        return None;
    }

    Some(TryTokenizeResult { token: None, width })
}
