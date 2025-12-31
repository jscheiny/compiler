use crate::lexer::Token;

pub trait ParserPredicate {
    fn is_match(&self, token: &Token) -> bool;
}
