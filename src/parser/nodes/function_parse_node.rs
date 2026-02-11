use std::{cell::OnceCell, collections::HashSet};

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

    pub fn check(&self, types: &mut TypeResolver) {
        self.check_params(types);
    }

    fn check_params(&self, types: &TypeResolver) {
        let mut param_names = HashSet::new();
        for param in self.parameters.iter() {
            if param_names.contains(param.id()) {
                println!(
                    "Type error: Duplicate parameter named `{}` of function `{}`",
                    param.id(),
                    self.id()
                );
            }
            param_names.insert(param.id().clone());
            param.check(types)
        }
    }

    pub fn get_type(&self, types: &TypeResolver) -> &FunctionType {
        self.resolved_type.get_or_init(|| self.get_type_impl(types))
    }

    fn get_type_impl(&self, types: &TypeResolver) -> FunctionType {
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
