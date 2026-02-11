use crate::{
    lexer::{IdentifierToken, Token},
    parser::{IdentifierNode, IdentifierType, ParseResult, SyntaxError, TokenStream},
};

pub fn identifier(
    tokens: &mut TokenStream,
    identifier_type: IdentifierType,
) -> ParseResult<IdentifierNode> {
    let token = tokens.peek();
    match token {
        Token::Identifier(IdentifierToken(identifier)) => {
            let identifier = identifier.clone();
            tokens.next();
            Ok(IdentifierNode(identifier))
        }
        Token::Keyword(keyword) => {
            let identifier = keyword.as_str().to_owned();
            tokens.push_error(SyntaxError::ExpectedIdentifier(identifier_type));
            tokens.next();
            Ok(IdentifierNode(identifier))
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedIdentifier(identifier_type))),
    }
}
