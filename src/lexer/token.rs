use crate::lexer::{
    CharacterLocation, IdentifierToken, IntegerLiteralToken, KeywordToken, OperatorToken,
    WhitespaceToken,
};

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(IdentifierToken),
    IntegerLiteral(IntegerLiteralToken),
    Operator(OperatorToken),
    Keyword(KeywordToken),
    Whitespace(WhitespaceToken),
}

#[derive(Debug, Clone)]
pub struct LocatedToken {
    pub token: Token,
    pub start: CharacterLocation,
    pub end: CharacterLocation,
}

pub trait TokenParse
where
    Self: Sized,
{
    fn try_tokenize(text: &str) -> Option<(Token, usize)>;
}
