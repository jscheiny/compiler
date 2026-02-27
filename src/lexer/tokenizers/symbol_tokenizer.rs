use strum::IntoEnumIterator;

use crate::lexer::{Symbol, Token, TokenWidth, Tokenizer, TryTokenizeResult};

pub struct SymbolTokenizer;

impl Tokenizer for SymbolTokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult> {
        for symbol in Symbol::iter() {
            let symbol_str = symbol.as_str();
            if text.starts_with(symbol_str) {
                return Some(TryTokenizeResult {
                    token: Some(Token::Symbol(symbol)),
                    width: TokenWidth::from(symbol_str),
                });
            }
        }
        None
    }
}
