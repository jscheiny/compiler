use std::{cell::OnceCell, collections::HashSet};

use crate::{
    checker::{FunctionType, TypeResolver},
    parser::{
        FunctionBodyNode, Identified, IdentifierNode, Node, NodeVec, ParameterNode, TypeNode,
    },
};

pub struct FunctionNode {
    identifier: Node<IdentifierNode>,
    parameters: NodeVec<ParameterNode>,
    return_type: Option<Node<TypeNode>>,
    body: Node<FunctionBodyNode>,
    resolved_type: OnceCell<FunctionType>,
}

impl FunctionNode {
    pub fn new(
        identifier: Node<IdentifierNode>,
        parameters: NodeVec<ParameterNode>,
        return_type: Option<Node<TypeNode>>,
        body: Node<FunctionBodyNode>,
    ) -> Self {
        Self {
            identifier,
            parameters,
            return_type,
            body,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self) {
        self.check_params();
    }

    fn check_params(&self) {
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

impl Identified for FunctionNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
