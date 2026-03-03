use strum::IntoEnumIterator;

use crate::lexer::{Keyword, Token, Tokenizer, TryTokenizeResult, try_tokenize_enum};

pub struct KeywordTokenizer;

impl Tokenizer for KeywordTokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult> {
        try_tokenize_enum(text, Keyword::iter(), Token::Keyword)
    }
}
