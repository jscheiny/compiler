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
    resolved_type: Option<FunctionType>,
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
            resolved_type: None,
        }
    }

    pub fn get_type(&mut self, types: &TypeResolver) -> FunctionType {
        match self.resolved_type.as_ref() {
            Some(resolved_type) => resolved_type.clone(),
            None => {
                let resolved_type = self.resolve_type(types);
                let result = resolved_type.clone();
                self.resolved_type = Some(resolved_type);
                result
            }
        }
    }

    fn resolve_type(&mut self, types: &TypeResolver) -> FunctionType {
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

impl Identified for FunctionParseNode {
    fn id(&self) -> &String {
        &self.identifier.id()
    }
}
