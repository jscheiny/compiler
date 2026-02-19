use crate::lexer::{KeywordToken, Symbol, Token};

pub trait Operator
where
    Self: Sized,
{
    fn from_token(token: &Token) -> Option<Self>;
    fn precedence(&self) -> i32;
}

#[derive(Copy, Clone, Debug)]
pub enum PostfixOperator {
    NullShortCircuit, // ?
}

impl Operator for PostfixOperator {
    fn from_token(token: &Token) -> Option<PostfixOperator> {
        match token {
            Token::Symbol(Symbol::QuestionMark) => Some(Self::NullShortCircuit),
            _ => None,
        }
    }

    fn precedence(&self) -> i32 {
        8
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PrefixOperator {
    Closure,    // .
    LogicalNot, // not
    Negative,   // -
}

impl Operator for PrefixOperator {
    fn from_token(token: &Token) -> Option<PrefixOperator> {
        match token {
            Token::Symbol(Symbol::Dot) => Some(Self::Closure),
            Token::Symbol(Symbol::Minus) => Some(Self::Negative),
            Token::Keyword(KeywordToken::Not) => Some(Self::LogicalNot),
            _ => None,
        }
    }

    fn precedence(&self) -> i32 {
        8
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    fn from_token(token: &Token) -> Option<BinaryOperator> {
        use KeywordToken as K;
        use Symbol as S;
        match token {
            Token::Symbol(operator) => match operator {
                S::PlusEqual => Some(Self::AddAssign),
                S::MinusEqual => Some(Self::SubtractAssign),
                S::TimesEqual => Some(Self::MultiplyAssign),
                S::SlashEqual => Some(Self::DivideAssign),
                S::PercentEqual => Some(Self::ModAssign),
                S::DoubleEqual => Some(Self::Equal),
                S::NotEqual => Some(Self::NotEqual),
                S::LessThan => Some(Self::LessThan),
                S::LessThanEqual => Some(Self::LessThanOrEqual),
                S::GreaterThan => Some(Self::GreaterThan),
                S::GreaterThanEqual => Some(Self::GreaterThanOrEqual),
                S::Dot => Some(Self::Access),
                S::ThickArrow => Some(Self::FunctionApplication),
                S::Comma => Some(Self::Comma),
                S::Colon => Some(Self::Type),
                S::Equal => Some(Self::Assign),
                S::Plus => Some(Self::Add),
                S::Minus => Some(Self::Subtract),
                S::Times => Some(Self::Multiply),
                S::Slash => Some(Self::Divide),
                S::Percent => Some(Self::Mod),
                _ => None,
            },
            Token::Keyword(keyword) => match keyword {
                K::And => Some(Self::LogicalAnd),
                K::Or => Some(Self::LogicalOr),
                _ => None,
            },
            _ => None,
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
