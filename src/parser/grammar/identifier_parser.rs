use crate::{
    lexer::{IdentifierToken, Token},
    parser::{IdentifierParseNode, IdentifierType, ParseResult, SyntaxError, TokenStream},
};

pub fn identifier(
    tokens: &mut TokenStream,
    identifier_type: IdentifierType,
) -> ParseResult<IdentifierParseNode> {
    let token = tokens.peek();
    match token {
        Token::Identifier(IdentifierToken(identifier)) => {
            let identifier = identifier.clone();
            tokens.next();
            Ok(IdentifierParseNode(identifier))
        }
        Token::Keyword(keyword) => {
            let identifier = keyword.as_str().to_owned();
            tokens.push_error(SyntaxError::ExpectedIdentifier(identifier_type));
            tokens.next();
            Ok(IdentifierParseNode(identifier))
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedIdentifier(identifier_type))),
    }
}
