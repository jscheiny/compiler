use crate::{
    lexer::{Keyword, Symbol, TokenMatch},
    parser::{
        FunctionSignatureNode, IdentifierType, ImplementationEntryNode,
        InterfaceImplementationNode, InterfaceNode, Node, ParseResult, SyntaxError, TokenStream,
        grammar::{end_statement, function_signature, nested_function},
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
    no_qualifiers(tokens);
    let signature = function_signature(tokens, IdentifierType::Interface)?;
    end_statement(tokens);
    Ok(signature)
}

pub fn interface_implementation(tokens: &mut TokenStream) -> ParseResult<ImplementationEntryNode> {
    let identifier = tokens.identifier(IdentifierType::Interface)?;
    if tokens.accept(Symbol::Semicolon) {
        Ok(ImplementationEntryNode::Interface(
            InterfaceImplementationNode {
                identifier,
                methods: None,
            },
        ))
    } else if tokens.accept(Symbol::OpenBrace) {
        let mut methods = vec![];
        while !tokens.accept(Symbol::CloseBrace) {
            no_qualifiers(tokens);
            methods.push(tokens.located(nested_function)?);
        }
        Ok(ImplementationEntryNode::Interface(
            InterfaceImplementationNode {
                identifier,
                methods: Some(methods),
            },
        ))
    } else {
        Err(tokens.make_error(SyntaxError::ExpectedMethods))
    }
}

fn no_qualifiers(tokens: &mut TokenStream) {
    if Keyword::Pub.matches(tokens.peek()) {
        tokens.push_error(SyntaxError::UnexpectedMethodSignatureQualifier(
            Keyword::Pub,
        ));
        tokens.next();
    }
}
