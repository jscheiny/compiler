use crate::parser::{ParameterParseNode, ParseNode, ParseNodeVec, TypeDefinitionParseNode};

#[derive(Debug)]
pub struct InterfaceDefinitionParseNode {
    pub identifier: ParseNode<String>,
    pub method_signatures: ParseNodeVec<MethodSignatureParseNode>,
}

#[derive(Debug)]
pub struct MethodSignatureParseNode {
    pub identifier: ParseNode<String>,
    pub parameters: ParseNodeVec<ParameterParseNode>,
    pub return_type: ParseNode<TypeDefinitionParseNode>,
}
