use crate::{
    lexer::OperatorToken,
    parser::{
        InterfaceDefinitionParseNode, ParseNode, MethodSignatureParseNode, TokenStream,
        grammar::{parameters, type_definition},
    },
};

pub fn interface(tokens: &mut TokenStream) -> Result<InterfaceDefinitionParseNode, ()> {
    tokens.next();
    let identifier = tokens.identifier()?;
    let method_signatures = tokens.located(method_signatures)?;

    Ok(InterfaceDefinitionParseNode {
        identifier,
        method_signatures,
    })
}

fn method_signatures(
    tokens: &mut TokenStream,
) -> Result<Vec<ParseNode<MethodSignatureParseNode>>, ()> {
    tokens.expect(&OperatorToken::OpenBrace)?;
    let mut methods = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        methods.push(tokens.located(method_signature)?);
    }
    Ok(methods)
}

fn method_signature(tokens: &mut TokenStream) -> Result<MethodSignatureParseNode, ()> {
    let identifier = tokens.identifier()?;
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
