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
            Token::Operator(OperatorToken::NullShortCircuit) => Some(Self::NullShortCircuit),
            _ => None,
        }
    }

    fn precedence(&self) -> i32 {
        10
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
            Token::Operator(OperatorToken::Access) => Some(Self::Closure),
            Token::Operator(OperatorToken::Subtract) => Some(Self::Negative),
            Token::Operator(OperatorToken::SelfRef) => Some(Self::SelfRef),
            Token::Keyword(KeywordToken::Not) => Some(Self::LogicalNot),
            _ => None,
        }
    }

    fn precedence(&self) -> i32 {
        9
    }
}

#[derive(Clone, Copy, Debug)]
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
    LogicalAnd,          // and
    LogicalOr,           // or
}

impl Operator for BinaryOperator {
    fn from_token(token: &Token) -> Option<BinaryOperator> {
        use KeywordToken as K;
        use OperatorToken as O;
        match token {
            Token::Operator(operator) => match operator {
                O::AddAssign => Some(Self::AddAssign),
                O::SubtractAssign => Some(Self::SubtractAssign),
                O::MultiplyAssign => Some(Self::MultiplyAssign),
                O::DivideAssign => Some(Self::DivideAssign),
                O::ModAssign => Some(Self::ModAssign),
                O::Equal => Some(Self::Equal),
                O::NotEqual => Some(Self::NotEqual),
                O::LessThanOrEqual => Some(Self::LessThanOrEqual),
                O::GreaterThanOrEqual => Some(Self::GreaterThanOrEqual),
                O::Access => Some(Self::Access),
                O::FunctionApplication => Some(Self::FunctionApplication),
                O::Assign => Some(Self::Assign),
                O::LessThan => Some(Self::LessThan),
                O::GreaterThan => Some(Self::GreaterThan),
                O::Add => Some(Self::Add),
                O::Subtract => Some(Self::Subtract),
                O::Multiply => Some(Self::Multiply),
                O::Divide => Some(Self::Divide),
                O::Mod => Some(Self::Mod),
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
            // Where does this go???
            Self::FunctionApplication => todo!(),
        }
    }
}
