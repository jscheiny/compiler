use crate::{
    lexer::{Keyword, Symbol, Token, TokenMatch},
    parser::{
        ExpressionNode, FunctionBodyNode, FunctionNode, FunctionSignatureNode,
        ImplementationEntryNode, ImplementationNode, MethodNode, NameType, Node, ParameterNode,
        ParseResult, SyntaxError, TokenStream,
        grammar::{
            BlockType, block, comma_separated_list, end_statement, expression,
            interface_implementation, type_definition,
        },
    },
};

pub fn implementation(tokens: &mut TokenStream) -> ParseResult<Option<Node<ImplementationNode>>> {
    if Symbol::OpenBrace.matches(tokens.peek()) {
        Ok(Some(tokens.located(implementation_impl)?))
    } else if tokens.accept(Symbol::Semicolon) {
        Ok(None)
    } else {
        tokens.push_error(SyntaxError::ExpectedMethods);
        Ok(None)
    }
}

fn implementation_impl(tokens: &mut TokenStream) -> ParseResult<ImplementationNode> {
    tokens.next();
    let mut entries = vec![];
    while !tokens.accept(Symbol::CloseBrace) {
        if tokens.accept(Keyword::Impl) {
            entries.push(tokens.located(interface_implementation)?);
        } else {
            entries.push(tokens.located(method)?);
        }
    }

    Ok(ImplementationNode::new(entries))
}

fn method(tokens: &mut TokenStream) -> ParseResult<ImplementationEntryNode> {
    let public = tokens.accept(Keyword::Pub);
    let function = tokens.located(nested_function)?;
    Ok(ImplementationEntryNode::Method(Box::new(MethodNode {
        public,
        function,
    })))
}

pub fn top_level_function(tokens: &mut TokenStream) -> ParseResult<FunctionNode> {
    function(tokens, true)
}

pub fn nested_function(tokens: &mut TokenStream) -> ParseResult<FunctionNode> {
    function(tokens, false)
}

fn function(tokens: &mut TokenStream, has_keyword: bool) -> ParseResult<FunctionNode> {
    if has_keyword {
        tokens.next();
    }
    let name_type = if has_keyword {
        NameType::Function
    } else {
        NameType::Method
    };
    let signature = function_signature(tokens, name_type)?;
    let body = tokens.located(function_body)?;
    Ok(FunctionNode::new(signature, body))
}

pub fn function_signature(
    tokens: &mut TokenStream,
    name_type: NameType,
) -> ParseResult<FunctionSignatureNode> {
    let name = tokens.name(name_type)?;
    let parameters = tokens.located(parameters)?;
    let return_type = if tokens.accept(Symbol::Colon) {
        Some(tokens.located(type_definition)?)
    } else {
        None
    };

    Ok(FunctionSignatureNode::new(name, parameters, return_type))
}

fn function_body(tokens: &mut TokenStream) -> ParseResult<FunctionBodyNode> {
    if tokens.accept(Symbol::SkinnyArrow) {
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
    use Symbol as S;
    let error = SyntaxError::ExpectedParameters;
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
    let name = tokens.name(NameType::Parameter)?;
    let error = SyntaxError::ExpectedType;
    match tokens.peek() {
        Token::Symbol(Symbol::Colon) => {
            tokens.next();
            let type_def = Some(tokens.located(type_definition)?);
            Ok(ParameterNode::new(name, type_def))
        }
        Token::Symbol(Symbol::Comma) | Token::Symbol(Symbol::CloseParen) => {
            tokens.push_error(error);
            Ok(ParameterNode::new(name, None))
        }
        _ => Err(tokens.make_error(error)),
    }
}
