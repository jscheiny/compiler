use crate::lexer::{Token, TokenWidth};

pub trait Tokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct TryTokenizeResult {
    pub token: Option<Token>,
    pub width: TokenWidth,
}
