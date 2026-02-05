use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::lexer::{Token, TokenMatch, TokenWidth, TryTokenizeResult};

#[derive(Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum OperatorToken {
    // Two character operators
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
    // One character operators
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

impl OperatorToken {
    fn as_str(&self) -> &str {
        match self {
            // Two character operators
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
            // One character operators
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
            Self::At => "^",
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

impl Display for OperatorToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TokenMatch for OperatorToken {
    fn matches(&self, token: &Token) -> bool {
        match token {
            Token::Operator(op) => *op == *self,
            _ => false,
        }
    }
}

pub fn try_tokenize_operator(text: &str) -> Option<TryTokenizeResult> {
    for operator in OperatorToken::iter() {
        let operator_str = operator.as_str();
        if text.starts_with(operator_str) {
            return Some(TryTokenizeResult {
                token: Some(Token::Operator(operator)),
                width: TokenWidth::from(operator_str),
            });
        }
    }
    None
}
