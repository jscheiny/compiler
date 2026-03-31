use std::fmt::Display;

use strum_macros::EnumIter;

use crate::lexer::{EnumToken, Token, TokenMatch};

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
pub enum Keyword {
    And,
    Bool,
    Break,
    Char,
    Continue,
    Else,
    Enum,
    False,
    Float,
    Fn,
    For,
    If,
    Impl,
    Int,
    Interface,
    Let,
    Match,
    Mut,
    Not,
    Or,
    Pub,
    Result,
    Return,
    SelfValue,
    SelfType,
    Struct,
    Then,
    True,
    Type,
    Void,
    While,
}

impl EnumToken for Keyword {
    fn as_str(&self) -> &str {
        match self {
            Self::And => "and",
            Self::Bool => "bool",
            Self::Break => "break",
            Self::Char => "char",
            Self::Continue => "continue",
            Self::Else => "else",
            Self::Enum => "enum",
            Self::False => "false",
            Self::Float => "float",
            Self::Fn => "fn",
            Self::For => "for",
            Self::If => "if",
            Self::Impl => "impl",
            Self::Int => "int",
            Self::Interface => "interface",
            Self::Let => "let",
            Self::Match => "match",
            Self::Mut => "mut",
            Self::Not => "not",
            Self::Or => "or",
            Self::Pub => "pub",
            Self::Result => "Result",
            Self::Return => "return",
            Self::SelfType => "Self",
            Self::SelfValue => "self",
            Self::Struct => "struct",
            Self::Then => "then",
            Self::True => "true",
            Self::Type => "type",
            Self::Void => "void",
            Self::While => "while",
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TokenMatch for Keyword {
    fn matches(&self, token: &Token) -> bool {
        match token {
            Token::Keyword(op) => *op == *self,
            _ => false,
        }
    }
}
