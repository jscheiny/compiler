use crate::lexer::{
    IdentifierToken, IntegerLiteralToken, KeywordToken, OperatorToken, TokenSpan, WhitespaceToken,
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
    pub span: TokenSpan,
}

pub trait TokenParse
where
    Self: Sized,
{
    fn try_tokenize(text: &str) -> Option<(Token, usize)>;
}
