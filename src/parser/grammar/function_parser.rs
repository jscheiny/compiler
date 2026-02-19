use crate::{
    lexer::{KeywordToken, Symbol, Token, TokenMatch},
    parser::{
        ExpressionNode, FunctionBodyNode, FunctionNode, IdentifierType, MethodNode, Node,
        ParameterNode, ParseResult, SyntaxError, TokenStream,
        grammar::{
            BlockType, block, comma_separated_list, end_statement, expression, type_definition,
        },
    },
};

pub fn methods(tokens: &mut TokenStream) -> ParseResult<Option<Node<Vec<Node<MethodNode>>>>> {
    if Symbol::OpenBrace.matches(tokens.peek()) {
        Ok(Some(tokens.located(methods_impl)?))
    } else if tokens.accept(&Symbol::Semicolon) {
        Ok(None)
    } else {
        tokens.push_error(SyntaxError::ExpectedMethods);
        Ok(None)
    }
}

fn methods_impl(tokens: &mut TokenStream) -> ParseResult<Vec<Node<MethodNode>>> {
    tokens.next();
    let mut methods = vec![];
    while !tokens.accept(&Symbol::CloseBrace) {
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
    let return_type = if tokens.accept(&Symbol::Colon) {
        Some(tokens.located(type_definition)?)
    } else {
        None
    };
    let body = tokens.located(function_body)?;
    Ok(FunctionNode::new(identifier, parameters, return_type, body))
}

fn function_body(tokens: &mut TokenStream) -> ParseResult<FunctionBodyNode> {
    if tokens.accept(&Symbol::SkinnyArrow) {
        let expression = expression(tokens)?;
        end_statement(tokens);
        Ok(FunctionBodyNode::Expression(expression))
    } else if Symbol::OpenBrace.matches(tokens.peek()) {
        Ok(FunctionBodyNode::Block(block(
            tokens,
            BlockType::Expression,
        )?))
    } else if Symbol::Semicolon.matches(tokens.peek()) {
        tokens.push_error(SyntaxError::ExpectedFunctionBody);
        tokens.next();
        Ok(FunctionBodyNode::Expression(ExpressionNode::Error))
    } else {
        Err(tokens.make_error(SyntaxError::ExpectedFunctionBody))
    }
}

pub fn parameters(tokens: &mut TokenStream) -> ParseResult<Vec<Node<ParameterNode>>> {
    let error = SyntaxError::ExpectedParameters;
    use Symbol as S;
    match tokens.peek() {
        Token::Symbol(S::OpenParen) => {
            tokens.next();
            let list = comma_separated_list(tokens, S::CloseParen, parameter)?;
            Ok(list)
        }
        Token::Symbol(S::SkinnyArrow) | Token::Symbol(S::OpenBrace) => {
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
        Token::Symbol(Symbol::Colon) => {
            tokens.next();
            let type_def = Some(tokens.located(type_definition)?);
            Ok(ParameterNode::new(identifier, type_def))
        }
        Token::Symbol(Symbol::Comma) | Token::Symbol(Symbol::CloseParen) => {
            tokens.push_error(error);
            Ok(ParameterNode::new(identifier, None))
        }
        _ => Err(tokens.make_error(error)),
    }
}
