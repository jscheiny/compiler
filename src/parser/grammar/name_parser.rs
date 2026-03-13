use crate::{
    lexer::{EnumToken, Token},
    parser::{NameType, ParseResult, SyntaxError, TokenStream},
};

pub fn name(tokens: &mut TokenStream, name_type: NameType) -> ParseResult<String> {
    let token = tokens.peek();
    match token {
        Token::Name(name) => {
            let name = name.clone();
            tokens.next();
            Ok(name)
        }
        Token::Keyword(keyword) => {
            let name = keyword.as_str().to_owned();
            tokens.push_error(SyntaxError::ExpectedName(name_type));
            tokens.next();
            Ok(name)
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedName(name_type))),
    }
}
