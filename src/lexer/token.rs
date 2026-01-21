use std::fmt::Display;

use crate::lexer::{
    IdentifierToken, IntegerLiteralToken, KeywordToken, OperatorToken, StringLiteralToken,
    TokenSpan, WhitespaceToken,
};

#[derive(Clone)]
pub enum Token {
    Identifier(IdentifierToken),
    IntegerLiteral(IntegerLiteralToken),
    StringLiteral(StringLiteralToken),
    Operator(OperatorToken),
    Keyword(KeywordToken),
    Whitespace(WhitespaceToken),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(identifier) => write!(f, "Identifier:{}", identifier),
            Self::IntegerLiteral(integer_literal) => write!(f, "Integer:{}", integer_literal),
            Self::StringLiteral(string_literal) => write!(f, "String:{}", string_literal),
            Self::Operator(operator) => write!(f, "Operator:{}", operator),
            Self::Keyword(keyword) => write!(f, "Keyword:{:?}", keyword),
            Self::Whitespace(_) => write!(f, "Whitespace"),
        }
    }
}

#[derive(Clone)]
pub struct LocatedToken {
    pub token: Token,
    pub span: TokenSpan,
}

impl Display for LocatedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.token, self.span)
    }
}

pub trait TokenParse
where
    Self: Sized,
{
    fn try_tokenize(text: &str) -> Option<(Token, usize)>;
}
