use crate::{
    lexer::{IdentifierToken, KeywordToken, Token},
    parser::{ParseResult, SyntaxError, TokenStream, TypeParseNode},
};

pub fn type_definition(tokens: &mut TokenStream) -> ParseResult<TypeParseNode> {
    let token = tokens.peek();
    match token {
        Token::Identifier(IdentifierToken(identifier)) => {
            let identifier = identifier.clone();
            tokens.next();
            Ok(TypeParseNode::UserDefined(identifier))
        }
        Token::Keyword(keyword) => match keyword {
            KeywordToken::Bool | KeywordToken::Int | KeywordToken::Float => {
                let keyword = *keyword;
                tokens.next();
                Ok(TypeParseNode::Primitive(keyword))
            }
            _ => Err(tokens.make_error(SyntaxError::ExpectedType)),
        },
        _ => Err(tokens.make_error(SyntaxError::ExpectedType)),
    }
}
