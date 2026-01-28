use crate::{
    lexer::{IdentifierToken, Token},
    parser::TokenStream,
};

pub fn identifier(tokens: &mut TokenStream) -> Result<String, ()> {
    if let Token::Identifier(IdentifierToken(identifier)) = tokens.peek() {
        let identifier = identifier.clone();
        tokens.next();
        Ok(identifier)
    } else {
        Err(())
    }
}
