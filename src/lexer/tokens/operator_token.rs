use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::lexer::{Token, TokenMatch};

#[derive(Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum OperatorToken {
    // Two character operators
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModAssign,
    Equal,
    LessThanOrEqual,
    GreaterThanOrEqual,
    FunctionDefinition,
    FunctionApplication,
    // One character operators
    EndStatement,
    Assign,
    LessThan,
    GreaterThan,
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Type,
    SelfRef,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    NullShortCircuit,
    Access,
    Comma,
}

impl OperatorToken {
    fn as_string(&self) -> &str {
        match self {
            // Two character operators
            Self::AddAssign => "+=",
            Self::SubtractAssign => "-=",
            Self::MultiplyAssign => "*=",
            Self::DivideAssign => "/=",
            Self::ModAssign => "%=",
            Self::Equal => "==",
            Self::LessThanOrEqual => "<=",
            Self::GreaterThanOrEqual => ">=",
            Self::FunctionDefinition => "->",
            Self::FunctionApplication => "=>",
            // One character operators
            Self::EndStatement => ";",
            Self::Assign => "=",
            Self::LessThan => "<",
            Self::GreaterThan => ">",
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Mod => "%",
            Self::Type => ":",
            Self::SelfRef => "^",
            Self::OpenParen => "(",
            Self::CloseParen => ")",
            Self::OpenBracket => "[",
            Self::CloseBracket => "]",
            Self::OpenBrace => "{",
            Self::CloseBrace => "}",
            Self::NullShortCircuit => "?",
            Self::Access => ".",
            Self::Comma => ",",
        }
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

impl Display for OperatorToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

pub fn try_tokenize_operator(text: &str) -> Option<(Token, usize)> {
    for operator in OperatorToken::iter() {
        let operator_str = operator.as_string();
        if text.starts_with(operator_str) {
            return Some((Token::Operator(operator), operator_str.len()));
        }
    }
    None
}
