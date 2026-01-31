use crate::{
    lexer::{IdentifierToken, Token},
    parser::{IdentifierParseNode, ParseResult, SyntaxErrorType, TokenStream},
};

pub fn identifier(tokens: &mut TokenStream) -> ParseResult<IdentifierParseNode> {
    identifier_with(tokens, SyntaxErrorType::ExpectedIdentifier)
}

pub fn identifier_with(
    tokens: &mut TokenStream,
    error: SyntaxErrorType,
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
            tokens.push_error(error);
            tokens.next();
            Ok(IdentifierParseNode(identifier))
        }
        _ => Err(tokens.make_error(error)),
    }
}
