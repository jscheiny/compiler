use crate::{
    lexer::Symbol,
    parser::{
        IdentifierType, ParseResult, SyntaxError, TokenStream, TypeAliasNode,
        grammar::{end_statement, type_definition},
    },
};

pub fn type_alias(tokens: &mut TokenStream) -> ParseResult<TypeAliasNode> {
    tokens.next();
    let identifier = tokens.identifier(IdentifierType::Function)?;
    tokens.expect(&Symbol::Equal, SyntaxError::ExpectedType)?;
    let type_def = tokens.located(type_definition)?;
    end_statement(tokens);
    Ok(TypeAliasNode::new(identifier, type_def))
}
