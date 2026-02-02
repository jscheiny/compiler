use crate::{
    lexer::{OperatorToken, Token},
    parser::{
        ExpectedSyntax, InterfaceDefinitionParseNode, MethodSignatureParseNode, ParseNode,
        ParseResult, SyntaxError, TokenStream,
        grammar::{identifier, parameters, type_definition},
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
    tokens.expect(&OperatorToken::Type, SyntaxError::Unimplemented)?;
    let return_type = tokens.located(type_definition)?;
    tokens.expect(&OperatorToken::EndStatement, SyntaxError::Unimplemented)?;
    Ok(MethodSignatureParseNode {
        identifier,
        parameters,
        return_type,
    })
}
