use crate::{
    lexer::OperatorToken,
    parser::{
        InterfaceDefinitionParseNode, MethodSignatureParseNode, ParseNode, ParseResult,
        TokenStream,
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
    tokens.expect(&OperatorToken::OpenBrace)?;
    let mut methods = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        methods.push(tokens.located(method_signature)?);
    }
    Ok(methods)
}

fn method_signature(tokens: &mut TokenStream) -> ParseResult<MethodSignatureParseNode> {
    let identifier = tokens.located(identifier)?;
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
