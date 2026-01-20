use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    lexer::{Token, TokenParse},
    parser::ParserPredicate,
};

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
    If,
}

impl KeywordToken {
    pub fn to_string(&self) -> &str {
        use KeywordToken as K;
        match self {
            K::Interface => "interface",
            K::Continue => "continue",
            K::Return => "return",
            K::Struct => "struct",
            K::Break => "break",
            K::Float => "float",
            K::Tuple => "tuple",
            K::While => "while",
            K::Bool => "bool",
            K::Else => "else",
            K::Enum => "enum",
            K::Int => "int",
            K::Let => "let",
            K::Mut => "mut",
            K::For => "for",
            K::Pub => "pub",
            K::If => "if",
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

impl ParserPredicate for KeywordToken {
    fn is_match(&self, token: &Token) -> bool {
        match token {
            Token::Keyword(op) => *op == *self,
            _ => false,
        }
    }
}
