use strum_macros::EnumIter;

use crate::{
    lexer::{Symbol, Token},
    parser::Operator,
};

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum PostfixOperator {
    NullShortCircuit, // ?
}

impl Operator for PostfixOperator {
    fn as_token(&self) -> Token {
        match self {
            Self::NullShortCircuit => Token::Symbol(Symbol::QuestionMark),
        }
    }

    fn precedence(&self) -> i32 {
        8
    }
}
