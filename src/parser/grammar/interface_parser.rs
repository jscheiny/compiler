use crate::{
    lexer::{OperatorToken, Token},
    parser::{
        IdentifierType, InterfaceDefinitionParseNode, MethodSignatureParseNode, ParseNode,
        ParseResult, SyntaxError, TokenStream, TypeParseNode,
        grammar::{end_statement, parameters, type_definition},
    },
};

pub fn interface(tokens: &mut TokenStream) -> ParseResult<InterfaceDefinitionParseNode> {
    tokens.next();
    let identifier = tokens.identifier(IdentifierType::Interface)?;
    let method_signatures = tokens.located(method_signatures)?;

    Ok(InterfaceDefinitionParseNode {
        identifier,
        method_signatures,
    })
}

fn method_signatures(
    tokens: &mut TokenStream,
) -> ParseResult<Vec<ParseNode<MethodSignatureParseNode>>> {
    match tokens.peek() {
        Token::Operator(OperatorToken::OpenBrace) => {
            tokens.next();
            let mut methods = vec![];
            while !tokens.accept(&OperatorToken::CloseBrace) {
                methods.push(tokens.located(method_signature)?);
            }
            Ok(methods)
        }
        Token::Operator(OperatorToken::EndStatement) => {
            tokens.push_error(SyntaxError::ExpectedMethodSignatures);
            tokens.next();
            Ok(vec![])
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedMethodSignatures)),
    }
}

fn method_signature(tokens: &mut TokenStream) -> ParseResult<MethodSignatureParseNode> {
    let identifier = tokens.identifier(IdentifierType::Method)?;
    let parameters = tokens.located(parameters)?;
    let return_type = return_type(tokens)?;
    end_statement(tokens);
    Ok(MethodSignatureParseNode {
        identifier,
        parameters,
        return_type,
    })
}

fn return_type(tokens: &mut TokenStream) -> ParseResult<Option<ParseNode<TypeParseNode>>> {
    let error = SyntaxError::ExpectedReturnType;
    match tokens.peek() {
        Token::Operator(OperatorToken::Type) => {
            tokens.next();
            Ok(Some(tokens.located(type_definition)?))
        }
        Token::Operator(OperatorToken::EndStatement) => {
            tokens.push_error(error);
            Ok(None)
        }
        _ => Err(tokens.make_error(error)),
    }
}
