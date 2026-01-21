use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    lexer::{Token, TokenParse},
    parser::ParserPredicate,
};

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
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
    DoubleQuote,
    SingleQuote,
    NullShortCircuit,
    Access,
    Comma,
}

impl OperatorToken {
    fn as_string(&self) -> &str {
        use OperatorToken as O;
        match self {
            // Two character operators
            O::AddAssign => "+=",
            O::SubtractAssign => "-=",
            O::MultiplyAssign => "*=",
            O::DivideAssign => "/=",
            O::ModAssign => "%=",
            O::Equal => "==",
            O::LessThanOrEqual => "<=",
            O::GreaterThanOrEqual => ">=",
            O::FunctionDefinition => "->",
            O::FunctionApplication => "=>",
            // One character operators
            O::EndStatement => ";",
            O::Assign => "=",
            O::LessThan => "<",
            O::GreaterThan => ">",
            O::Add => "+",
            O::Subtract => "-",
            O::Multiply => "*",
            O::Divide => "/",
            O::Mod => "%",
            O::Type => ":",
            O::SelfRef => "^",
            O::OpenParen => "(",
            O::CloseParen => ")",
            O::OpenBracket => "[",
            O::CloseBracket => "]",
            O::OpenBrace => "{",
            O::CloseBrace => "}",
            O::DoubleQuote => "\"",
            O::SingleQuote => "'",
            O::NullShortCircuit => "?",
            O::Access => ".",
            O::Comma => ",",
        }
    }

    pub fn maybe_match(text: &str) -> Option<OperatorToken> {
        OperatorToken::iter().find(|operator| text.starts_with(operator.as_string()))
    }
}

impl TokenParse for OperatorToken {
    fn try_tokenize(text: &str) -> Option<(Token, usize)> {
        for operator in OperatorToken::iter() {
            let operator_str = operator.as_string();
            if text.starts_with(operator_str) {
                return Some((Token::Operator(operator), operator_str.len()));
            }
        }
        None
    }
}

impl ParserPredicate for OperatorToken {
    fn is_match(&self, token: &Token) -> bool {
        match token {
            Token::Operator(op) => *op == *self,
            _ => false,
        }
    }
}
