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
    pub fn new(
        identifier: ParseNode<IdentifierParseNode>,
        parameters: ParseNodeVec<ParameterParseNode>,
        return_type: Option<ParseNode<TypeParseNode>>,
        body: ParseNode<FunctionBodyParseNode>,
    ) -> Self {
        Self {
            identifier,
            parameters,
            return_type,
            body,
        }
    }

    pub fn resolve_type(&mut self, types: &TypeResolver) -> FunctionType {
        let parameters = self
            .parameters
            .value
            .iter_mut()
            .map(|parameter| parameter.value.get_type(types))
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
