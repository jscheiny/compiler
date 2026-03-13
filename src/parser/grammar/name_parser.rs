use crate::{
    lexer::{EnumToken, Token},
    parser::{NameNode, NameType, ParseResult, SyntaxError, TokenStream},
};

pub fn name(tokens: &mut TokenStream, identifier_type: NameType) -> ParseResult<NameNode> {
    let token = tokens.peek();
    match token {
        Token::Name(name) => {
            let name = name.clone();
            tokens.next();
            Ok(NameNode(name))
        }
        Token::Keyword(keyword) => {
            let name = keyword.as_str().to_owned();
            tokens.push_error(SyntaxError::ExpectedName(identifier_type));
            tokens.next();
            Ok(NameNode(name))
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedName(identifier_type))),
    }
}
