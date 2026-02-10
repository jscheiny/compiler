use crate::lexer::KeywordToken;

#[derive(Clone, Copy)]
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
