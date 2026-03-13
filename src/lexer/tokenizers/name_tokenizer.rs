use crate::lexer::{Token, TokenWidth, Tokenizer, TryTokenizeResult};

pub struct NameTokenizer;

impl Tokenizer for NameTokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult> {
        let mut width = TokenWidth::new();
        let mut name = String::from("");
        for character in text.chars() {
            if !character.is_alphanumeric() && character != '_' {
                break;
            }

            name.push(character);
            width.add_char(character);
        }

        let len = name.len();
        if len == 0 {
            return None;
        }

        Some(TryTokenizeResult {
            token: Some(Token::Name(name)),
            width,
        })
    }
}
