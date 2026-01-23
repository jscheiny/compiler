use crate::{
    lexer::{OperatorToken, TokenMatch},
    parser::{
        FunctionBodyParseNode, FunctionDefintionParseNode, ParameterParseNode, ParseNode,
        TokenStream,
        grammar::{block, expression, type_definition, utils::comma_separated_list},
    },
};

pub fn top_level_function(tokens: &mut TokenStream) -> Result<FunctionDefintionParseNode, ()> {
    function(tokens, true)
}

pub fn nested_function(tokens: &mut TokenStream) -> Result<FunctionDefintionParseNode, ()> {
    function(tokens, false)
}

fn function(tokens: &mut TokenStream, has_keyword: bool) -> Result<FunctionDefintionParseNode, ()> {
    if has_keyword {
        tokens.next();
    }
    let identifier = tokens.identifier()?;
    let parameters = tokens.located(parameters)?;
    let return_type = if tokens.accept(&OperatorToken::Type) {
        Some(tokens.located(type_definition)?)
    } else {
        None
    };
    let body = tokens.located(function_body)?;
    Ok(FunctionDefintionParseNode {
        identifier,
        parameters,
        return_type,
        body,
    })
}

fn function_body(tokens: &mut TokenStream) -> Result<FunctionBodyParseNode, ()> {
    if tokens.accept(&OperatorToken::FunctionDefinition) {
        let expression = expression(tokens)?;
        tokens.expect(&OperatorToken::EndStatement)?;
        Ok(FunctionBodyParseNode::Expression(expression))
    } else if OperatorToken::OpenBrace.matches(tokens.peek()) {
        Ok(FunctionBodyParseNode::Block(block(tokens)?))
    } else {
        Err(())
    }
}

pub fn parameters(tokens: &mut TokenStream) -> Result<Vec<ParseNode<ParameterParseNode>>, ()> {
    tokens.expect(&OperatorToken::OpenParen)?;
    let list = comma_separated_list(tokens, OperatorToken::CloseParen, parameter)?;
    Ok(list)
}

fn parameter(tokens: &mut TokenStream) -> Result<ParameterParseNode, ()> {
    let identifier = tokens.identifier()?;
    tokens.expect(&OperatorToken::Type)?;
    let type_def = tokens.located(type_definition)?;

    Ok(ParameterParseNode {
        identifier,
        type_def,
    })
}
