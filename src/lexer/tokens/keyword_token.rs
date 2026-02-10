use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::lexer::{Token, TokenMatch, TokenWidth, TryTokenizeResult};

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
pub enum KeywordToken {
    Continue,
    Return,
    Struct,
    Break,
    Float,
    While,
    Bool,
    Else,
    Enum,
    Then,
    Type,
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

impl KeywordToken {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Continue => "continue",
            Self::Return => "return",
            Self::Struct => "struct",
            Self::Break => "break",
            Self::Float => "float",
            Self::While => "while",
            Self::Bool => "bool",
            Self::Else => "else",
            Self::Enum => "enum",
            Self::Then => "then",
            Self::Type => "type",
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

impl Display for KeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TokenMatch for KeywordToken {
    fn matches(&self, token: &Token) -> bool {
        match token {
            Token::Keyword(op) => *op == *self,
            _ => false,
        }
    }
}

pub fn try_tokenize_keyword(text: &str) -> Option<TryTokenizeResult> {
    for keyword in KeywordToken::iter() {
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
