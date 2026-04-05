use std::fmt::Display;

use strum_macros::EnumIter;

use crate::lexer::{EnumToken, Token, TokenMatch};

#[derive(Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum Symbol {
    At,
    CloseBrace,
    CloseBracket,
    CloseParen,
    Colon,
    Comma,
    Dot,
    DoubleColon,
    DoubleEqual,
    Elipsis,
    Equal,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Minus,
    MinusEqual,
    NotEqual,
    OpenBrace,
    OpenBracket,
    OpenParen,
    Percent,
    PercentEqual,
    Plus,
    PlusEqual,
    QuestionMark,
    Semicolon,
    SkinnyArrow,
    Slash,
    SlashEqual,
    ThickArrow,
    Times,
    TimesEqual,
}

impl EnumToken for Symbol {
    fn as_str(&self) -> &str {
        match self {
            Self::At => "@",
            Self::CloseBrace => "}",
            Self::CloseBracket => "]",
            Self::CloseParen => ")",
            Self::Colon => ":",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::DoubleColon => "::",
            Self::DoubleEqual => "==",
            Self::Elipsis => "...",
            Self::Equal => "=",
            Self::GreaterThan => ">",
            Self::GreaterThanEqual => ">=",
            Self::LessThan => "<",
            Self::LessThanEqual => "<=",
            Self::Minus => "-",
            Self::MinusEqual => "-=",
            Self::NotEqual => "!=",
            Self::OpenBrace => "{",
            Self::OpenBracket => "[",
            Self::OpenParen => "(",
            Self::Percent => "%",
            Self::PercentEqual => "%=",
            Self::Plus => "+",
            Self::PlusEqual => "+=",
            Self::QuestionMark => "?",
            Self::Semicolon => ";",
            Self::SkinnyArrow => "->",
            Self::Slash => "/",
            Self::SlashEqual => "/=",
            Self::ThickArrow => "=>",
            Self::Times => "*",
            Self::TimesEqual => "*=",
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
