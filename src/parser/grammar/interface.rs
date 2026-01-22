use crate::{
    lexer::OperatorToken,
    parser::{
        InterfaceDefinitionParseNode, LocatedNodeVec, MethodSignatureParseNode, ParseResult,
        TokenTraverser,
        grammar::{parameters, type_definition},
    },
};

pub fn interface(tokens: &mut TokenTraverser) -> ParseResult<InterfaceDefinitionParseNode> {
    let span = tokens.start_span();
    tokens.next();
    let identifier = tokens.located_identifier().ok_or(())?;
    let method_signatures = method_signatures(tokens)?;

    Ok(span.close(
        tokens,
        InterfaceDefinitionParseNode {
            identifier,
            method_signatures,
        },
    ))
}

fn method_signatures(
    tokens: &mut TokenTraverser,
) -> Result<LocatedNodeVec<MethodSignatureParseNode>, ()> {
    let span = tokens.start_span();
    tokens.expect(&OperatorToken::OpenBrace)?;
    let mut methods = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        methods.push(method_signature(tokens)?);
    }
    Ok(span.close(tokens, methods))
}

fn method_signature(tokens: &mut TokenTraverser) -> ParseResult<MethodSignatureParseNode> {
    let span = tokens.start_span();
    let identifier = tokens.located_identifier().ok_or(())?;
    let parameters = parameters(tokens)?;
    tokens.expect(&OperatorToken::Type)?;
    let return_type = type_definition(tokens)?;
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(span.close(
        tokens,
        MethodSignatureParseNode {
            identifier,
            parameters,
            return_type,
        },
    ))
}
