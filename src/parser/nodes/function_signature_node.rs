use std::cell::OnceCell;

use crate::{
    checker::{FunctionType, Scope, Type},
    parser::{IdentifierNode, Node, NodeVec, ParameterNode, TypeNode},
};

pub struct FunctionSignatureNode {
    pub identifier: Node<IdentifierNode>,
    pub parameters: NodeVec<ParameterNode>,
    pub return_type: Option<Node<TypeNode>>,
    resolved_type: OnceCell<FunctionType>,
}

impl FunctionSignatureNode {
    pub fn new(
        identifier: Node<IdentifierNode>,
        parameters: NodeVec<ParameterNode>,
        return_type: Option<Node<TypeNode>>,
    ) -> Self {
        Self {
            identifier,
            parameters,
            return_type,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, scope: &Scope) -> &FunctionType {
        self.resolved_type.get_or_init(|| self.get_type_impl(scope))
    }

    fn get_type_impl(&self, scope: &Scope) -> FunctionType {
        let parameters = self
            .parameters
            .value
            .iter()
            .map(|parameter| parameter.get_type(scope))
            .cloned()
            .collect();

        let return_type = self
            .return_type
            .as_ref()
            .map_or(Type::Void, |return_type| return_type.get_type(scope));

        FunctionType {
            parameters,
            return_type: Box::new(return_type),
        }
    }
}
