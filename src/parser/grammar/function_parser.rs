use crate::{
    lexer::{KeywordToken, OperatorToken, TokenMatch},
    parser::{
        ExpectedSyntax, FunctionBodyParseNode, FunctionDefintionParseNode, MethodParseNode,
        ParameterParseNode, ParseNode, ParseResult, SyntaxError, TokenStream,
        grammar::{block, comma_separated_list, expression, identifier, type_definition},
    },
};

pub fn methods(
    tokens: &mut TokenStream,
) -> ParseResult<Option<ParseNode<Vec<ParseNode<MethodParseNode>>>>> {
    if OperatorToken::OpenBrace.matches(tokens.peek()) {
        Ok(Some(tokens.located(methods_impl)?))
    } else if tokens.accept(&OperatorToken::EndStatement) {
        Ok(None)
    } else {
        tokens.push_error(SyntaxError::Expected(ExpectedSyntax::Methods));
        Ok(None)
    }
}

fn methods_impl(tokens: &mut TokenStream) -> ParseResult<Vec<ParseNode<MethodParseNode>>> {
    tokens.next();
    let mut methods = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        methods.push(tokens.located(method)?);
    }
    Ok(methods)
}

fn method(tokens: &mut TokenStream) -> ParseResult<MethodParseNode> {
    let public = tokens.accept(&KeywordToken::Pub);
    let function = tokens.located(nested_function)?;
    Ok(MethodParseNode { public, function })
}

pub fn top_level_function(tokens: &mut TokenStream) -> ParseResult<FunctionDefintionParseNode> {
    function(tokens, true)
}

fn nested_function(tokens: &mut TokenStream) -> ParseResult<FunctionDefintionParseNode> {
    function(tokens, false)
}

fn function(
    tokens: &mut TokenStream,
    has_keyword: bool,
) -> ParseResult<FunctionDefintionParseNode> {
    if has_keyword {
        tokens.next();
    }
    let identifier = tokens.located(identifier)?;
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

fn function_body(tokens: &mut TokenStream) -> ParseResult<FunctionBodyParseNode> {
    if tokens.accept(&OperatorToken::FunctionDefinition) {
        let expression = expression(tokens)?;
        tokens.expect(&OperatorToken::EndStatement)?;
        Ok(FunctionBodyParseNode::Expression(expression))
    } else if OperatorToken::OpenBrace.matches(tokens.peek()) {
        Ok(FunctionBodyParseNode::Block(block(tokens)?))
    } else {
        Err(tokens.make_error(SyntaxError::Unimplemented))
    }
}

pub fn parameters(tokens: &mut TokenStream) -> ParseResult<Vec<ParseNode<ParameterParseNode>>> {
    tokens.expect(&OperatorToken::OpenParen)?;
    let list = comma_separated_list(tokens, OperatorToken::CloseParen, parameter)?;
    Ok(list)
}

fn parameter(tokens: &mut TokenStream) -> ParseResult<ParameterParseNode> {
    let identifier = tokens.located(identifier)?;
    tokens.expect(&OperatorToken::Type)?;
    let type_def = tokens.located(type_definition)?;

    Ok(ParameterParseNode {
        identifier,
        type_def,
    })
}
