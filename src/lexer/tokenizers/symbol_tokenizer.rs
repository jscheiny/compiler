use strum::IntoEnumIterator;

use crate::lexer::{Symbol, Token, Tokenizer, TryTokenizeResult, try_tokenize_enum};

pub struct SymbolTokenizer;

impl Tokenizer for SymbolTokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult> {
        try_tokenize_enum(text, Symbol::iter(), Token::Symbol)
    }
}
