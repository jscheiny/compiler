use crate::{
    lexer::{Keyword, Symbol, TokenMatch},
    parser::{
        ImplementationEntryNode, InterfaceImplementationNode, InterfaceNode, MethodNode,
        MethodSignatureNode, NameType, Node, ParseResult, SyntaxError, TokenStream,
        grammar::{end_statement, function_signature, method_instance_kind, nested_function},
    },
};

pub fn interface(tokens: &mut TokenStream) -> ParseResult<InterfaceNode> {
    tokens.next();
    let name = tokens.name(NameType::Interface)?;
    let method_signatures = tokens.located(method_signatures)?;

    Ok(InterfaceNode::new(name, method_signatures))
}

pub fn method_signatures(tokens: &mut TokenStream) -> ParseResult<Vec<Node<MethodSignatureNode>>> {
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

pub fn method_signature(tokens: &mut TokenStream) -> ParseResult<MethodSignatureNode> {
    no_qualifiers(tokens);
    let instance_kind = method_instance_kind(tokens);
    let signature = function_signature(tokens, NameType::Interface)?;
    end_statement(tokens);
    Ok(MethodSignatureNode {
        instance_kind,
        signature,
    })
}

pub fn interface_implementation(tokens: &mut TokenStream) -> ParseResult<ImplementationEntryNode> {
    let name = tokens.name(NameType::Interface)?;
    if tokens.accept(Symbol::Semicolon) {
        Ok(ImplementationEntryNode::Interface(
            InterfaceImplementationNode {
                name,
                methods: None,
            },
        ))
    } else if tokens.accept(Symbol::OpenBrace) {
        let mut methods = vec![];
        while !tokens.accept(Symbol::CloseBrace) {
            no_qualifiers(tokens);
            methods.push(tokens.located(implemented_method)?);
        }
        Ok(ImplementationEntryNode::Interface(
            InterfaceImplementationNode {
                name,
                methods: Some(methods),
            },
        ))
    } else {
        Err(tokens.make_error(SyntaxError::ExpectedMethods))
    }
}

fn implemented_method(tokens: &mut TokenStream) -> ParseResult<MethodNode> {
    let instance_kind = method_instance_kind(tokens);
    let function = tokens.located(nested_function)?;
    Ok(MethodNode {
        private: false, // Interface methods are always public
        instance_kind,
        function,
    })
}

fn no_qualifiers(tokens: &mut TokenStream) {
    if Keyword::Impl.matches(tokens.peek()) {
        tokens.push_error(SyntaxError::UnexpectedMethodSignatureQualifier(
            Keyword::Impl,
        ));
        tokens.next();
    }
}
