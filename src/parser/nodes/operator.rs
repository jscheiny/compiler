use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::lexer::{Keyword, Symbol, Token};

pub trait Operator
where
    Self: Sized + IntoEnumIterator,
{
    fn as_token(&self) -> Token;
    fn from_token(token: &Token) -> Option<Self> {
        for operator in Self::iter() {
            if operator.as_token() == *token {
                return Some(operator);
            }
        }
        None
    }
    fn precedence(&self) -> i32;
}

#[derive(Copy, Clone, Debug, EnumIter)]
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

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum PrefixOperator {
    Closure,    // .
    LogicalNot, // not
    Negative,   // -
}

impl Operator for PrefixOperator {
    fn as_token(&self) -> Token {
        match self {
            Self::Closure => Token::Symbol(Symbol::Dot),
            Self::Negative => Token::Symbol(Symbol::Minus),
            Self::LogicalNot => Token::Keyword(Keyword::Not),
        }
    }

    fn precedence(&self) -> i32 {
        8
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
pub enum BinaryOperator {
    Add,                 // +
    AddAssign,           // +=
    Subtract,            // -
    SubtractAssign,      // -=
    Multiply,            // *
    MultiplyAssign,      // *=
    Divide,              // /
    DivideAssign,        // /=
    Mod,                 // %
    ModAssign,           // %=
    Assign,              // =
    Equal,               // ==
    NotEqual,            // !=
    LessThan,            // <
    LessThanOrEqual,     // <=
    GreaterThan,         // >
    GreaterThanOrEqual,  // >=
    Access,              // .
    FunctionApplication, // =>
    Comma,               // ,
    Type,                // :
    LogicalAnd,          // and
    LogicalOr,           // or
}

impl Operator for BinaryOperator {
    fn as_token(&self) -> Token {
        use Symbol as S;
        match self {
            Self::AddAssign => Token::Symbol(S::PlusEqual),
            Self::SubtractAssign => Token::Symbol(S::MinusEqual),
            Self::MultiplyAssign => Token::Symbol(S::TimesEqual),
            Self::DivideAssign => Token::Symbol(S::SlashEqual),
            Self::ModAssign => Token::Symbol(S::PercentEqual),
            Self::Equal => Token::Symbol(S::DoubleEqual),
            Self::NotEqual => Token::Symbol(S::NotEqual),
            Self::LessThan => Token::Symbol(S::LessThan),
            Self::LessThanOrEqual => Token::Symbol(S::LessThanEqual),
            Self::GreaterThan => Token::Symbol(S::GreaterThan),
            Self::GreaterThanOrEqual => Token::Symbol(S::GreaterThanEqual),
            Self::Access => Token::Symbol(S::Dot),
            Self::FunctionApplication => Token::Symbol(S::ThickArrow),
            Self::Comma => Token::Symbol(S::Comma),
            Self::Type => Token::Symbol(S::Colon),
            Self::Assign => Token::Symbol(S::Equal),
            Self::Add => Token::Symbol(S::Plus),
            Self::Subtract => Token::Symbol(S::Minus),
            Self::Multiply => Token::Symbol(S::Times),
            Self::Divide => Token::Symbol(S::Slash),
            Self::Mod => Token::Symbol(S::Percent),
            Self::LogicalAnd => Token::Keyword(Keyword::And),
            Self::LogicalOr => Token::Keyword(Keyword::Or),
        }
    }

    fn precedence(&self) -> i32 {
        match self {
            // Access (and function calls not expressed here)
            Self::Access => 9,
            // Function application should bind slightly less tight than function calls
            Self::FunctionApplication => 8,
            // Multiplicative
            Self::Multiply | Self::Divide | Self::Mod => 7,
            // Additive
            Self::Add | Self::Subtract => 6,
            // Relational
            Self::LessThan
            | Self::LessThanOrEqual
            | Self::GreaterThan
            | Self::GreaterThanOrEqual => 5,
            // Equality
            Self::Equal | Self::NotEqual => 4,
            // Logical and
            Self::LogicalAnd => 3,
            // Logical or
            Self::LogicalOr => 2,
            // Assignment
            Self::AddAssign
            | Self::SubtractAssign
            | Self::MultiplyAssign
            | Self::DivideAssign
            | Self::ModAssign
            | Self::Assign => 1,
            // Comma / type
            Self::Comma | Self::Type => 0,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Associativity {
    Left,
    Right,
}

impl BinaryOperator {
    pub fn associativity(&self) -> Associativity {
        match self {
            Self::AddAssign
            | Self::SubtractAssign
            | Self::MultiplyAssign
            | Self::DivideAssign
            | Self::ModAssign
            | Self::Assign
            | Self::Comma => Associativity::Right,
            _ => Associativity::Left,
        }
    }
}
