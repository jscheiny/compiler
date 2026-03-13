use crate::{
    lexer::{EnumToken, Token},
    parser::{IdentifierType, NameNode, ParseResult, SyntaxError, TokenStream},
};

pub fn name(tokens: &mut TokenStream, identifier_type: IdentifierType) -> ParseResult<NameNode> {
    let token = tokens.peek();
    match token {
        Token::Name(name) => {
            let name = name.clone();
            tokens.next();
            Ok(NameNode(name))
        }
        Token::Keyword(keyword) => {
            let name = keyword.as_str().to_owned();
            tokens.push_error(SyntaxError::ExpectedIdentifier(identifier_type));
            tokens.next();
            Ok(NameNode(name))
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedIdentifier(identifier_type))),
    }
}
