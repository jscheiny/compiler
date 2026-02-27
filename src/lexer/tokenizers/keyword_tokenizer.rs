use strum::IntoEnumIterator;

use crate::lexer::{Keyword, Token, TokenWidth, Tokenizer, TryTokenizeResult};

pub struct KeywordTokenizer;

impl Tokenizer for KeywordTokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult> {
        for keyword in Keyword::iter() {
            let keyword_str = keyword.as_str();
            if text.starts_with(keyword_str) {
                return Some(TryTokenizeResult {
                    token: Some(Token::Keyword(keyword)),
                    width: TokenWidth::from(keyword_str),
                });
            }
        }
        None
    }
}
