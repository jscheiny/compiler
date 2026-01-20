use crate::{
    lexer::OperatorToken,
    parser::{
        FunctionBodyParseNode, FunctionDefintionParseNode, ParameterParseNode, TokenTraverser,
        grammar::{expression, statement, type_definition, utils::comma_separated_list},
    },
};

pub fn function(tokens: &mut TokenTraverser) -> Result<FunctionDefintionParseNode, ()> {
    let identifier = tokens.identifier().ok_or(())?;
    let parameters = parameters(tokens)?;
    let return_type = if tokens.accept(&OperatorToken::Type) {
        Some(type_definition(tokens)?)
    } else {
        None
    };
    let body = function_body(tokens)?;
    Ok(FunctionDefintionParseNode {
        identifier,
        parameters,
        return_type,
        body,
    })
}

fn function_body(tokens: &mut TokenTraverser) -> Result<FunctionBodyParseNode, ()> {
    if tokens.accept(&OperatorToken::FunctionDefinition) {
        let expression = expression(tokens)?;
        tokens.expect(&OperatorToken::EndStatement)?;
        Ok(FunctionBodyParseNode::Expression(expression))
    } else if tokens.accept(&OperatorToken::OpenBrace) {
        let mut statements = vec![];
        while !tokens.accept(&OperatorToken::CloseBrace) {
            statements.push(statement(tokens)?);
        }
        Ok(FunctionBodyParseNode::Block(statements))
    } else {
        Err(())
    }
}

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
