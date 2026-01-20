use crate::{
    lexer::OperatorToken,
    parser::{
        InterfaceDefinitionParseNode, MethodSignatureParseNode, TokenTraverser,
        grammar::{parameters, type_definition},
    },
};

pub fn interface(tokens: &mut TokenTraverser) -> Result<InterfaceDefinitionParseNode, ()> {
    let identifier = tokens.identifier().ok_or(())?;
    tokens.expect(&OperatorToken::OpenBrace)?;
    let method_signatures = method_signatures(tokens)?;
    Ok(InterfaceDefinitionParseNode {
        identifier,
        method_signatures,
    })
}

fn method_signatures(tokens: &mut TokenTraverser) -> Result<Vec<MethodSignatureParseNode>, ()> {
    let mut methods = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        methods.push(method_signature(tokens)?);
    }
    Ok(methods)
}

fn method_signature(tokens: &mut TokenTraverser) -> Result<MethodSignatureParseNode, ()> {
    let identifier = tokens.identifier().ok_or(())?;
    let parameters = parameters(tokens)?;
    tokens.expect(&OperatorToken::Type)?;
    let return_type = type_definition(tokens)?;
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(MethodSignatureParseNode {
        identifier,
        parameters,
        return_type,
    })
}
