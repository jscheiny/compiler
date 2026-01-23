use crate::lexer::Token;

pub trait TokenMatch {
    fn matches(&self, token: &Token) -> bool;
}
