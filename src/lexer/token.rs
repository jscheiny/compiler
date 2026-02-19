use std::fmt::Display;

use crate::lexer::{
    CharacterSpan, IdentifierToken, IntegerLiteralToken, KeywordToken, StringLiteralToken, Symbol,
};

#[derive(Clone)]
pub struct LocatedToken {
    pub token: Token,
    pub span: CharacterSpan,
}

impl Display for LocatedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.token, self.span)
    }
}

#[derive(Clone)]
pub enum Token {
    Identifier(IdentifierToken),
    IntegerLiteral(IntegerLiteralToken),
    StringLiteral(StringLiteralToken),
    Symbol(Symbol),
    Keyword(KeywordToken),
    EndOfFile,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(identifier) => write!(f, "Identifier:{}", identifier),
            Self::IntegerLiteral(integer_literal) => write!(f, "Integer:{}", integer_literal),
            Self::StringLiteral(string_literal) => write!(f, "String:{}", string_literal),
            Self::Symbol(symbol) => write!(f, "Symbol:{}", symbol),
            Self::Keyword(keyword) => write!(f, "Keyword:{}", keyword),
            Self::EndOfFile => write!(f, "[EOF]"),
        }
    }
}
