use crate::{
    lexer::OperatorToken,
    parser::{
        InterfaceDefinitionParseNode, LocatedNode, MethodSignatureParseNode, TokenTraverser,
        grammar::{parameters, type_definition},
    },
};

pub fn interface(tokens: &mut TokenTraverser) -> Result<InterfaceDefinitionParseNode, ()> {
    tokens.next();
    let identifier = tokens.identifier().ok_or(())?;
    let method_signatures = tokens.located(method_signatures)?;

    Ok(InterfaceDefinitionParseNode {
        identifier,
        method_signatures,
    })
}

fn method_signatures(
    tokens: &mut TokenTraverser,
) -> Result<Vec<LocatedNode<MethodSignatureParseNode>>, ()> {
    tokens.expect(&OperatorToken::OpenBrace)?;
    let mut methods = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        methods.push(tokens.located(method_signature)?);
    }
    Ok(methods)
}

fn method_signature(tokens: &mut TokenTraverser) -> Result<MethodSignatureParseNode, ()> {
    let identifier = tokens.identifier().ok_or(())?;
    let parameters = tokens.located(parameters)?;
    tokens.expect(&OperatorToken::Type)?;
    let return_type = tokens.located(type_definition)?;
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(MethodSignatureParseNode {
        identifier,
        parameters,
        return_type,
    })
}
