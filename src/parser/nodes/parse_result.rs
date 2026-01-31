use crate::parser::SyntaxError;

pub type ParseResult<T> = Result<T, SyntaxError>;
