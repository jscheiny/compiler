use crate::parser::{
    IdentifierParseNode, ParameterParseNode, ParseNode, ParseNodeVec, TokenSpan, Traverse,
    TypeParseNode,
};

pub struct InterfaceDefinitionParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub method_signatures: ParseNodeVec<MethodSignatureParseNode>,
}

impl Traverse for InterfaceDefinitionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("Interface.identifier", self.identifier.span);
        visit("Interface.method_signatures", self.method_signatures.span);
        for method_signature in self.method_signatures.value.iter() {
            method_signature.traverse("Interface.method_signature", visit);
        }
    }
}

pub struct MethodSignatureParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub parameters: ParseNodeVec<ParameterParseNode>,
    pub return_type: ParseNode<TypeParseNode>,
}

impl Traverse for MethodSignatureParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("MethodSignature.identifier", self.identifier.span);
        visit("MethodSignature.parameters", self.parameters.span);
        for parameter in self.parameters.value.iter() {
            parameter.traverse("MethodSignature.parameter", visit);
        }
        self.return_type.traverse("MethodSignature.return", visit);
    }
}
