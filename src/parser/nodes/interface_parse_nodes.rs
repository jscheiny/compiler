use crate::parser::{
    ParameterParseNode, ParseNode, ParseNodeVec, TokenSpan, Traverse, TypeDefinitionParseNode,
};

#[derive(Debug)]
pub struct InterfaceDefinitionParseNode {
    pub identifier: ParseNode<String>,
    pub method_signatures: ParseNodeVec<MethodSignatureParseNode>,
}

impl Traverse for InterfaceDefinitionParseNode {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        visit(self.identifier.span);
        visit(self.method_signatures.span);
        for method_signature in self.method_signatures.value.iter() {
            method_signature.traverse(visit);
        }
    }
}

#[derive(Debug)]
pub struct MethodSignatureParseNode {
    pub identifier: ParseNode<String>,
    pub parameters: ParseNodeVec<ParameterParseNode>,
    pub return_type: ParseNode<TypeDefinitionParseNode>,
}

impl Traverse for MethodSignatureParseNode {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        visit(self.identifier.span);
        visit(self.parameters.span);
        for parameter in self.parameters.value.iter() {
            parameter.traverse(visit);
        }
        self.return_type.traverse(visit);
    }
}
