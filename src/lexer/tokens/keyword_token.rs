use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::lexer::{Token, TokenMatch, TokenParse};

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum KeywordToken {
    Interface,
    Continue,
    Return,
    Struct,
    Break,
    Float,
    Tuple,
    While,
    Bool,
    Else,
    Enum,
    Int,
    Let,
    Mut,
    Pub,
    For,
    Fn,
    If,
}

impl KeywordToken {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Interface => "interface",
            Self::Continue => "continue",
            Self::Return => "return",
            Self::Struct => "struct",
            Self::Break => "break",
            Self::Float => "float",
            Self::Tuple => "tuple",
            Self::While => "while",
            Self::Bool => "bool",
            Self::Else => "else",
            Self::Enum => "enum",
            Self::Int => "int",
            Self::Let => "let",
            Self::Mut => "mut",
            Self::For => "for",
            Self::Pub => "pub",
            Self::Fn => "fn",
            Self::If => "if",
        }
    }
}

impl TokenParse for KeywordToken {
    fn try_tokenize(text: &str) -> Option<(Token, usize)> {
        for keyword in KeywordToken::iter() {
            let keyword_str = keyword.to_string();
            if text.starts_with(keyword_str) {
                return Some((Token::Keyword(keyword), keyword_str.len()));
            }
        }
        None
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
