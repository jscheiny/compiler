use crate::{
    lexer::{IdentifierToken, Token},
    parser::{IdentifierParseNode, TokenStream},
};

pub fn identifier(tokens: &mut TokenStream) -> Result<IdentifierParseNode, ()> {
    if let Token::Identifier(IdentifierToken(identifier)) = tokens.peek() {
        let identifier = identifier.clone();
        tokens.next();
        Ok(IdentifierParseNode(identifier))
    } else {
        Err(())
    }
}
