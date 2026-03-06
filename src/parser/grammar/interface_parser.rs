use crate::{
    lexer::{Keyword, Symbol},
    parser::{
        FunctionSignatureNode, IdentifierType, InterfaceNode, Node, ParseResult, SyntaxError,
        TokenStream,
        grammar::{function_parser::function_signature, statement_parser::end_statement},
    },
};

pub fn interface(tokens: &mut TokenStream) -> ParseResult<InterfaceNode> {
    tokens.next();
    let identifier = tokens.identifier(IdentifierType::Interface)?;
    let method_signatures = tokens.located(method_signatures)?;

    Ok(InterfaceNode::new(identifier, method_signatures))
}

pub fn method_signatures(
    tokens: &mut TokenStream,
) -> ParseResult<Vec<Node<FunctionSignatureNode>>> {
    if tokens.accept(Symbol::Semicolon) {
        tokens.push_error(SyntaxError::ExpectedMethodSignatures);
        return Ok(vec![]);
    }

    tokens.expect(Symbol::OpenBrace, SyntaxError::ExpectedMethodSignatures)?;

    let mut method_signatures = vec![];
    while !tokens.accept(Symbol::CloseBrace) {
        method_signatures.push(tokens.located(method_signature)?);
    }

    Ok(method_signatures)
}

pub fn method_signature(tokens: &mut TokenStream) -> ParseResult<FunctionSignatureNode> {
    if tokens.accept(Keyword::Pub) {
        tokens.push_error(SyntaxError::UnexpectedMethodSignatureQualifier(
            Keyword::Pub,
        ));
    }

    let signature = function_signature(tokens, IdentifierType::Interface)?;
    end_statement(tokens);
    Ok(signature)
}
