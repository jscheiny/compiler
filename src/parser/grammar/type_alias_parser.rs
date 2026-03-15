use crate::{
    lexer::Symbol,
    parser::{
        NameType, ParseResult, SyntaxError, TokenStream, TypeAliasNode,
        grammar::{end_statement, type_definition, type_parameter_list},
    },
};

pub fn type_alias(tokens: &mut TokenStream) -> ParseResult<TypeAliasNode> {
    tokens.next();
    let name = tokens.name(NameType::Function)?;
    let type_parameters = if tokens.accept(Symbol::OpenBracket) {
        Some(tokens.located(type_parameter_list)?)
    } else {
        None
    };

    tokens.expect(Symbol::Equal, SyntaxError::ExpectedType)?;

    let type_def = tokens.located(type_definition)?;
    end_statement(tokens);

    Ok(TypeAliasNode::new(name, type_parameters, type_def))
}
