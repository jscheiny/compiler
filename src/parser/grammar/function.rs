use crate::{
    lexer::OperatorToken,
    parser::{
        FunctionBodyParseNode, FunctionDefintionParseNode, LocatedNode, ParameterParseNode,
        ParseResult, TokenTraverser,
        grammar::{expression, statement, type_definition, utils::comma_separated_list},
    },
};

pub fn function(
    tokens: &mut TokenTraverser,
    has_keyword: bool,
) -> ParseResult<FunctionDefintionParseNode> {
    let span = tokens.start_span();
    if has_keyword {
        tokens.next();
    }
    let identifier = tokens.identifier().ok_or(())?;
    let parameters = parameters(tokens)?;
    let return_type = if tokens.accept(&OperatorToken::Type) {
        Some(type_definition(tokens)?)
    } else {
        None
    };
    let body = function_body(tokens)?;
    Ok(span.close(
        tokens,
        FunctionDefintionParseNode {
            identifier,
            parameters,
            return_type,
            body,
        },
    ))
}

fn function_body(tokens: &mut TokenTraverser) -> ParseResult<FunctionBodyParseNode> {
    let span = tokens.start_span();
    if tokens.accept(&OperatorToken::FunctionDefinition) {
        let expression = expression(tokens)?;
        tokens.expect(&OperatorToken::EndStatement)?;
        Ok(span.close(tokens, FunctionBodyParseNode::Expression(expression)))
    } else if tokens.accept(&OperatorToken::OpenBrace) {
        let mut statements = vec![];
        while !tokens.accept(&OperatorToken::CloseBrace) {
            statements.push(statement(tokens)?);
        }
        Ok(span.close(tokens, FunctionBodyParseNode::Block(statements)))
    } else {
        Err(())
    }
}

pub fn parameters(
    tokens: &mut TokenTraverser,
) -> ParseResult<Vec<LocatedNode<ParameterParseNode>>> {
    let span = tokens.start_span();
    tokens.expect(&OperatorToken::OpenParen)?;
    let list = comma_separated_list(tokens, OperatorToken::CloseParen, parameter)?;
    Ok(span.close(tokens, list))
}

fn parameter(tokens: &mut TokenTraverser) -> ParseResult<ParameterParseNode> {
    let span = tokens.start_span();
    let identifier = tokens.identifier().ok_or(())?;
    tokens.expect(&OperatorToken::Type)?;
    let type_def = type_definition(tokens)?;

    Ok(span.close(
        tokens,
        ParameterParseNode {
            identifier,
            type_def,
        },
    ))
}
