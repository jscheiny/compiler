use crate::{
    checker::{FunctionType, TypeResolver},
    parser::{
        FunctionBodyParseNode, IdentifierParseNode, ParameterParseNode, ParseNode, ParseNodeVec,
        TypeParseNode,
    },
};

pub struct FunctionParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub parameters: ParseNodeVec<ParameterParseNode>,
    pub return_type: Option<ParseNode<TypeParseNode>>,
    pub body: ParseNode<FunctionBodyParseNode>,
}

impl FunctionParseNode {
    pub fn resolve_type(&self, types: &TypeResolver) -> FunctionType {
        let parameters = self
            .parameters
            .value
            .iter()
            .map(|parameter| parameter.value.resolve_type(types))
            .collect();

        let return_type = self
            .return_type
            .as_ref()
            .map(|rt| Box::new(rt.value.resolve_type(types)));

        FunctionType {
            parameters,
            return_type,
        }
    }
}
