use crate::{
    lexer::{IdentifierToken, Token},
    parser::{IdentifierParseNode, SyntaxError, SyntaxErrorType, TokenStream},
};

pub fn identifier(tokens: &mut TokenStream) -> Result<IdentifierParseNode, ()> {
    let token = tokens.peek();
    match token {
        Token::Identifier(IdentifierToken(identifier)) => {
            let identifier = identifier.clone();
            tokens.next();
            Ok(IdentifierParseNode(identifier))
        }
        Token::Keyword(keyword) => {
            let identifier = keyword.to_string().to_owned();
            tokens.error(SyntaxError {
                span: tokens.current_span(),
                kind: SyntaxErrorType::ExpectedIdentifier,
            });
            tokens.next();
            Ok(IdentifierParseNode(identifier))
        }
        _ => Err(()),
    }
}
