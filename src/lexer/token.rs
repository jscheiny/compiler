use std::fmt::Display;

use crate::lexer::{CharacterSpan, Keyword, Symbol};

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

#[derive(Clone, PartialEq, Eq)]
pub enum Token {
    CharacterLiteral(String),
    Identifier(String),
    IntegerLiteral(i64),
    StringLiteral(String),
    Symbol(Symbol),
    Keyword(Keyword),
    EndOfFile,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CharacterLiteral(literal) => write!(f, "{}", literal),
            Self::Identifier(identifier) => write!(f, "{}", identifier),
            Self::IntegerLiteral(literal) => write!(f, "{}", literal),
            Self::StringLiteral(literal) => write!(f, "\"{}\"", literal),
            Self::Symbol(symbol) => write!(f, "{}", symbol),
            Self::Keyword(keyword) => write!(f, "{}", keyword),
            Self::EndOfFile => write!(f, "[EOF]"),
        }
    }
}
