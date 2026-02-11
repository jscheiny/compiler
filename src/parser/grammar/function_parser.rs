use crate::{
    lexer::{KeywordToken, OperatorToken, Token, TokenMatch},
    parser::{
        ExpressionNode, FunctionBodyNode, FunctionNode, IdentifierType, MethodNode, ParameterNode,
        ParseNode, ParseResult, SyntaxError, TokenStream,
        grammar::{block, comma_separated_list, end_statement, expression, type_definition},
    },
};

pub fn methods(
    tokens: &mut TokenStream,
) -> ParseResult<Option<ParseNode<Vec<ParseNode<MethodNode>>>>> {
    if OperatorToken::OpenBrace.matches(tokens.peek()) {
        Ok(Some(tokens.located(methods_impl)?))
    } else if tokens.accept(&OperatorToken::Semicolon) {
        Ok(None)
    } else {
        tokens.push_error(SyntaxError::ExpectedMethods);
        Ok(None)
    }
}

fn methods_impl(tokens: &mut TokenStream) -> ParseResult<Vec<ParseNode<MethodNode>>> {
    tokens.next();
    let mut methods = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        methods.push(tokens.located(method)?);
    }
    Ok(methods)
}

fn method(tokens: &mut TokenStream) -> ParseResult<MethodNode> {
    let public = tokens.accept(&KeywordToken::Pub);
    let function = tokens.located(nested_function)?;
    Ok(MethodNode { public, function })
}

pub fn top_level_function(tokens: &mut TokenStream) -> ParseResult<FunctionNode> {
    function(tokens, true)
}

fn nested_function(tokens: &mut TokenStream) -> ParseResult<FunctionNode> {
    function(tokens, false)
}

fn function(tokens: &mut TokenStream, has_keyword: bool) -> ParseResult<FunctionNode> {
    if has_keyword {
        tokens.next();
    }
    let identifier_type = if has_keyword {
        IdentifierType::Function
    } else {
        IdentifierType::Method
    };
    let identifier = tokens.identifier(identifier_type)?;
    let parameters = tokens.located(parameters)?;
    let return_type = if tokens.accept(&OperatorToken::Colon) {
        Some(tokens.located(type_definition)?)
    } else {
        None
    };
    let body = tokens.located(function_body)?;
    Ok(FunctionNode::new(identifier, parameters, return_type, body))
}

fn function_body(tokens: &mut TokenStream) -> ParseResult<FunctionBodyNode> {
    if tokens.accept(&OperatorToken::SkinnyArrow) {
        let expression = expression(tokens)?;
        end_statement(tokens);
        Ok(FunctionBodyNode::Expression(expression))
    } else if OperatorToken::OpenBrace.matches(tokens.peek()) {
        Ok(FunctionBodyNode::Block(block(tokens)?))
    } else if OperatorToken::Semicolon.matches(tokens.peek()) {
        tokens.push_error(SyntaxError::ExpectedFunctionBody);
        tokens.next();
        Ok(FunctionBodyNode::Expression(ExpressionNode::Error))
    } else {
        Err(tokens.make_error(SyntaxError::ExpectedFunctionBody))
    }
}

pub fn parameters(tokens: &mut TokenStream) -> ParseResult<Vec<ParseNode<ParameterNode>>> {
    let error = SyntaxError::ExpectedParameters;
    use OperatorToken as O;
    match tokens.peek() {
        Token::Operator(O::OpenParen) => {
            tokens.next();
            let list = comma_separated_list(tokens, O::CloseParen, parameter)?;
            Ok(list)
        }
        Token::Operator(O::SkinnyArrow) | Token::Operator(O::OpenBrace) => {
            tokens.push_error(error);
            Ok(vec![])
        }
        _ => Err(tokens.make_error(error)),
    }
}

fn parameter(tokens: &mut TokenStream) -> ParseResult<ParameterNode> {
    let identifier = tokens.identifier(IdentifierType::Parameter)?;
    let error = SyntaxError::ExpectedType;
    match tokens.peek() {
        Token::Operator(OperatorToken::Colon) => {
            tokens.next();
            let type_def = Some(tokens.located(type_definition)?);
            Ok(ParameterNode::new(identifier, type_def))
        }
        Token::Operator(OperatorToken::Comma) | Token::Operator(OperatorToken::CloseParen) => {
            tokens.push_error(error);
            Ok(ParameterNode::new(identifier, None))
        }
        _ => Err(tokens.make_error(error)),
    }
}
