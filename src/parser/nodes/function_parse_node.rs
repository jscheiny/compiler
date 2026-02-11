use std::cell::OnceCell;

use crate::{
    checker::{FunctionType, TypeResolver},
    parser::{
        FunctionBodyParseNode, Identified, IdentifierParseNode, ParameterParseNode, ParseNode,
        ParseNodeVec, TypeParseNode,
    },
};

pub struct FunctionParseNode {
    identifier: ParseNode<IdentifierParseNode>,
    parameters: ParseNodeVec<ParameterParseNode>,
    return_type: Option<ParseNode<TypeParseNode>>,
    body: ParseNode<FunctionBodyParseNode>,
    resolved_type: OnceCell<FunctionType>,
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
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, types: &TypeResolver) -> &FunctionType {
        self.resolved_type.get_or_init(|| self.resolve_type(types))
    }

    fn resolve_type(&self, types: &TypeResolver) -> FunctionType {
        let parameters = self
            .parameters
            .value
            .iter()
            .map(|parameter| parameter.get_type(types))
            .cloned()
            .collect();

        let return_type = self
            .return_type
            .as_ref()
            .map(|rt| Box::new(rt.get_type(types)));

        FunctionType {
            parameters,
            return_type,
        }
    }
}

impl Identified for FunctionParseNode {
    fn id(&self) -> &String {
        &self.identifier.id()
    }
}
