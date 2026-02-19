use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::lexer::{Token, TokenMatch, TokenWidth, TryTokenizeResult};

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
pub enum Keyword {
    Continue,
    Return,
    Struct,
    Break,
    False,
    Float,
    While,
    Bool,
    Else,
    Enum,
    Then,
    True,
    Type,
    Void,
    And,
    Int,
    Let,
    Mut,
    Not,
    Pub,
    For,
    Fn,
    If,
    Or,
}

impl Keyword {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Continue => "continue",
            Self::Return => "return",
            Self::Struct => "struct",
            Self::Break => "break",
            Self::False => "false",
            Self::Float => "float",
            Self::While => "while",
            Self::Bool => "bool",
            Self::Else => "else",
            Self::Enum => "enum",
            Self::Then => "then",
            Self::True => "true",
            Self::Type => "type",
            Self::Void => "void",
            Self::And => "and",
            Self::Int => "int",
            Self::Let => "let",
            Self::Mut => "mut",
            Self::Not => "not",
            Self::For => "for",
            Self::Pub => "pub",
            Self::Fn => "fn",
            Self::If => "if",
            Self::Or => "or",
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

pub fn try_tokenize_keyword(text: &str) -> Option<TryTokenizeResult> {
    for keyword in Keyword::iter() {
        let keyword_str = keyword.as_str();
        if text.starts_with(keyword_str) {
            return Some(TryTokenizeResult {
                token: Some(Token::Keyword(keyword)),
                width: TokenWidth::from(keyword_str),
            });
        }
    }
    None
}
