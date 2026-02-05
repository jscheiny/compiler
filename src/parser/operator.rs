use crate::lexer::{KeywordToken, OperatorToken, Token};

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
            Token::Operator(OperatorToken::QuestionMark) => Some(Self::NullShortCircuit),
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
    SelfRef,    // @
}

impl Operator for PrefixOperator {
    fn from_token(token: &Token) -> Option<PrefixOperator> {
        match token {
            Token::Operator(OperatorToken::Dot) => Some(Self::Closure),
            Token::Operator(OperatorToken::Minus) => Some(Self::Negative),
            Token::Operator(OperatorToken::At) => Some(Self::SelfRef),
            Token::Keyword(KeywordToken::Not) => Some(Self::LogicalNot),
            _ => None,
        }
    }

    fn precedence(&self) -> i32 {
        9
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
    LogicalAnd,          // and
    LogicalOr,           // or
}

impl Operator for BinaryOperator {
    fn from_token(token: &Token) -> Option<BinaryOperator> {
        use KeywordToken as K;
        use OperatorToken as O;
        match token {
            Token::Operator(operator) => match operator {
                O::PlusEqual => Some(Self::AddAssign),
                O::MinusEqual => Some(Self::SubtractAssign),
                O::TimesEqual => Some(Self::MultiplyAssign),
                O::SlashEqual => Some(Self::DivideAssign),
                O::PercentEqual => Some(Self::ModAssign),
                O::DoubleEqual => Some(Self::Equal),
                O::NotEqual => Some(Self::NotEqual),
                O::LessThan => Some(Self::LessThan),
                O::LessThanEqual => Some(Self::LessThanOrEqual),
                O::GreaterThan => Some(Self::GreaterThan),
                O::GreaterThanEqual => Some(Self::GreaterThanOrEqual),
                O::Dot => Some(Self::Access),
                O::ThickArrow => Some(Self::FunctionApplication),
                O::Comma => Some(Self::Comma),
                O::Equal => Some(Self::Assign),
                O::Plus => Some(Self::Add),
                O::Minus => Some(Self::Subtract),
                O::Times => Some(Self::Multiply),
                O::Slash => Some(Self::Divide),
                O::Percent => Some(Self::Mod),
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
            // Access
            Self::Access => 8,
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
            // Comma
            Self::Comma => 0,
            // Where does this go???
            Self::FunctionApplication => todo!(),
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
