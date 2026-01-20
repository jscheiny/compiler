use crate::{lexer::OperatorToken, parser::{ParameterParseNode, TokenTraverser, grammar::type_definition}};

pub fn parameter(tokens: &mut TokenTraverser) -> Result<ParameterParseNode, ()> {
    let identifier = tokens.identifier().ok_or(())?;
    tokens.expect(&OperatorToken::Type)?;
    let type_def = type_definition(tokens)?;

    Ok(ParameterParseNode {
        identifier,
        type_def,
    })
}
