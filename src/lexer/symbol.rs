use std::fmt::Display;

use strum_macros::EnumIter;

use crate::lexer::{Token, TokenMatch};

#[derive(Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum Symbol {
    // Two character symbols
    PlusEqual,
    MinusEqual,
    TimesEqual,
    SlashEqual,
    PercentEqual,
    DoubleEqual,
    NotEqual,
    LessThanEqual,
    GreaterThanEqual,
    SkinnyArrow,
    ThickArrow,
    // One character symbols
    Semicolon,
    Equal,
    LessThan,
    GreaterThan,
    Plus,
    Minus,
    Times,
    Slash,
    Percent,
    Colon,
    At,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    QuestionMark,
    Dot,
    Comma,
}

impl Symbol {
    pub fn as_str(&self) -> &str {
        match self {
            // Two character symbols
            Self::PlusEqual => "+=",
            Self::MinusEqual => "-=",
            Self::TimesEqual => "*=",
            Self::SlashEqual => "/=",
            Self::PercentEqual => "%=",
            Self::DoubleEqual => "==",
            Self::NotEqual => "!=",
            Self::LessThanEqual => "<=",
            Self::GreaterThanEqual => ">=",
            Self::SkinnyArrow => "->",
            Self::ThickArrow => "=>",
            // One character symbols
            Self::Semicolon => ";",
            Self::Equal => "=",
            Self::LessThan => "<",
            Self::GreaterThan => ">",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Times => "*",
            Self::Slash => "/",
            Self::Percent => "%",
            Self::Colon => ":",
            Self::At => "@",
            Self::OpenParen => "(",
            Self::CloseParen => ")",
            Self::OpenBracket => "[",
            Self::CloseBracket => "]",
            Self::OpenBrace => "{",
            Self::CloseBrace => "}",
            Self::QuestionMark => "?",
            Self::Dot => ".",
            Self::Comma => ",",
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TokenMatch for Symbol {
    fn matches(&self, token: &Token) -> bool {
        match token {
            Token::Symbol(op) => *op == *self,
            _ => false,
        }
    }
}
