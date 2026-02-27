use crate::lexer::{TokenWidth, Tokenizer, TryTokenizeResult};

pub struct WhitespaceTokenizer;

impl Tokenizer for WhitespaceTokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult> {
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
}
