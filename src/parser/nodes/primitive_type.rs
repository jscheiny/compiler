use std::fmt::Display;

use crate::lexer::KeywordToken;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimitiveType {
    Bool,
    Float,
    Int,
}

impl PrimitiveType {
    pub fn from_token(keyword: KeywordToken) -> Option<PrimitiveType> {
        match keyword {
            KeywordToken::Bool => Some(Self::Bool),
            KeywordToken::Float => Some(Self::Float),
            KeywordToken::Int => Some(Self::Int),
            _ => None,
        }
    }
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Bool => KeywordToken::Bool,
            Self::Float => KeywordToken::Float,
            Self::Int => KeywordToken::Int,
        };
        write!(f, "{}", keyword)
    }
}
