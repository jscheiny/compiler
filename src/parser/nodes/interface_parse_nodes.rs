use crate::parser::{LocatedNode, LocatedNodeVec, ParameterParseNode, TypeDefinitionParseNode};

#[derive(Debug)]
pub struct InterfaceDefinitionParseNode {
    pub identifier: LocatedNode<String>,
    pub method_signatures: LocatedNodeVec<MethodSignatureParseNode>,
}

#[derive(Debug)]
pub struct MethodSignatureParseNode {
    pub identifier: LocatedNode<String>,
    pub parameters: LocatedNodeVec<ParameterParseNode>,
    pub return_type: LocatedNode<TypeDefinitionParseNode>,
}
