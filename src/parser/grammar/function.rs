use crate::{
    lexer::OperatorToken,
    parser::{
        ParameterParseNode, TokenTraverser,
        grammar::{type_definition, utils::comma_separated_list},
    },
};

pub fn parameters(tokens: &mut TokenTraverser) -> Result<Vec<ParameterParseNode>, ()> {
    tokens.expect(&OperatorToken::OpenParen)?;
    comma_separated_list(tokens, OperatorToken::CloseParen, parameter)
}

fn parameter(tokens: &mut TokenTraverser) -> Result<ParameterParseNode, ()> {
    let identifier = tokens.identifier().ok_or(())?;
    tokens.expect(&OperatorToken::Type)?;
    let type_def = type_definition(tokens)?;

    Ok(ParameterParseNode {
        identifier,
        type_def,
    })
}
