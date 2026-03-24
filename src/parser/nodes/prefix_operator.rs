use strum_macros::EnumIter;

use crate::{
    lexer::{Keyword, Symbol, Token},
    parser::Operator,
};

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum PrefixOperator {
    LogicalNot, // not
    Negative,   // -
}

impl Operator for PrefixOperator {
    fn as_token(&self) -> Token {
        match self {
            Self::Negative => Token::Symbol(Symbol::Minus),
            Self::LogicalNot => Token::Keyword(Keyword::Not),
        }
    }

    fn precedence(&self) -> i32 {
        8
    }
}
