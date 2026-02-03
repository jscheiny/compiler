use crate::{
    lexer::{OperatorToken, Token},
    parser::{
        ExpectedSyntax, InterfaceDefinitionParseNode, MethodSignatureParseNode, ParseNode,
        ParseResult, SyntaxError, TokenStream, TypeParseNode,
        grammar::{identifier, parameters, statement_parser::end_statement, type_definition},
    },
};

pub fn interface(tokens: &mut TokenStream) -> ParseResult<InterfaceDefinitionParseNode> {
    tokens.next();
    let identifier = tokens.located(identifier)?;
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
            tokens.push_error(SyntaxError::Expected(ExpectedSyntax::MethodSignatures));
            tokens.next();
            Ok(vec![])
        }
        _ => Err(tokens.make_error(SyntaxError::Expected(ExpectedSyntax::MethodSignatures))),
    }
}

fn method_signature(tokens: &mut TokenStream) -> ParseResult<MethodSignatureParseNode> {
    let identifier = tokens.located(identifier)?;
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
    let error = SyntaxError::Expected(ExpectedSyntax::ReturnType);
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
