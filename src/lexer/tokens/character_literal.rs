use crate::lexer::{Token, TokenWidth, Tokenizer, TryTokenizeResult};

const SINGLE_QUOTE: char = '\'';
const ESCAPE: char = '\\';

pub struct CharacterLiteralTokenizer;

// TODO combine implementation with string literal?
impl Tokenizer for CharacterLiteralTokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult> {
        if !text.starts_with(SINGLE_QUOTE) {
            return None;
        }

        let text = &text[1..];
        let mut skip_endquote = false;
        let mut has_endquote = false;
        let mut width = TokenWidth::new();
        width.add_char(SINGLE_QUOTE);
        for character in text.chars() {
            if character == SINGLE_QUOTE && !skip_endquote {
                has_endquote = true;
                break;
            }

            skip_endquote = character == ESCAPE;
            width.add_char(character);
        }

        if !has_endquote {
            return None;
        }

        let character = text[..width.bytes].to_string();
        width.add_char(SINGLE_QUOTE);

        Some(TryTokenizeResult {
            token: Some(Token::CharacterLiteral(character)),
            width,
        })
    }
}
