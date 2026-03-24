use strum::IntoEnumIterator;

use crate::lexer::Token;

pub trait Operator
where
    Self: Sized + IntoEnumIterator,
{
    fn as_token(&self) -> Token;
    fn precedence(&self) -> i32;

    fn from_token(token: &Token) -> Option<Self> {
        Self::iter().find(|operator| operator.as_token() == *token)
    }
}
