use crate::lexer::{Token, TokenWidth, Tokenizer, TryTokenizeResult};

const DOUBLE_QUOTE: char = '"';
const ESCAPE: char = '\\';

pub struct StringLiteralTokenizer;

impl Tokenizer for StringLiteralTokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult> {
        if !text.starts_with(DOUBLE_QUOTE) {
            return None;
        }

        let mut skip_endquote = false;
        let mut has_endquote = false;
        let mut width = TokenWidth::new();
        width.add_char(DOUBLE_QUOTE);

        for character in text[1..].chars() {
            if character == DOUBLE_QUOTE && !skip_endquote {
                has_endquote = true;
                break;
            }

            if character == '\n' {
                return None;
            }

            skip_endquote = character == ESCAPE;
            width.add_char(character);
        }

        if !has_endquote {
            return None;
        }

        let string = text[1..width.bytes].to_string();
        width.add_char(DOUBLE_QUOTE);
        Some(TryTokenizeResult {
            token: Some(Token::StringLiteral(string)),
            width,
        })
    }
}
