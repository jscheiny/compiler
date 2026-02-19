use std::fmt::Display;

use crate::lexer::Keyword;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimitiveType {
    Bool,
    Float,
    Int,
}

impl PrimitiveType {
    pub fn from_token(keyword: Keyword) -> Option<PrimitiveType> {
        match keyword {
            Keyword::Bool => Some(Self::Bool),
            Keyword::Float => Some(Self::Float),
            Keyword::Int => Some(Self::Int),
            _ => None,
        }
    }
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Bool => Keyword::Bool,
            Self::Float => Keyword::Float,
            Self::Int => Keyword::Int,
        };
        write!(f, "{}", keyword)
    }
}
