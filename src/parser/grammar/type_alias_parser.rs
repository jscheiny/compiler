use crate::{
    lexer::OperatorToken,
    parser::{
        IdentifierType, ParseResult, SyntaxError, TokenStream, TypeAliasParseNode,
        grammar::{end_statement, type_definition},
    },
};

pub fn type_alias(tokens: &mut TokenStream) -> ParseResult<TypeAliasParseNode> {
    tokens.next();
    let identifier = tokens.identifier(IdentifierType::Function)?;
    tokens.expect(&OperatorToken::Equal, SyntaxError::ExpectedType)?;
    let type_def = tokens.located(type_definition)?;
    end_statement(tokens);
    Ok(TypeAliasParseNode::new(identifier, type_def))
}
