use crate::parser::LocatedSyntaxError;

pub type ParseResult<T> = Result<T, LocatedSyntaxError>;
