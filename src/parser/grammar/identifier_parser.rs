use crate::{
    lexer::{EnumToken, Token},
    parser::{IdentifierType, NameNode, ParseResult, SyntaxError, TokenStream},
};

pub fn identifier(
    tokens: &mut TokenStream,
    identifier_type: IdentifierType,
) -> ParseResult<NameNode> {
    let token = tokens.peek();
    match token {
        Token::Identifier(identifier) => {
            let identifier = identifier.clone();
            tokens.next();
            Ok(NameNode(identifier))
        }
        Token::Keyword(keyword) => {
            let identifier = keyword.as_str().to_owned();
            tokens.push_error(SyntaxError::ExpectedIdentifier(identifier_type));
            tokens.next();
            Ok(NameNode(identifier))
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedIdentifier(identifier_type))),
    }
}
