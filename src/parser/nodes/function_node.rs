use std::{cell::OnceCell, collections::HashSet};

use crate::{
    checker::{FunctionType, Scope, Type, TypeResolver},
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

    pub fn check(&self, types: &TypeResolver, parent_scope: Box<Scope>) -> Box<Scope> {
        let return_type = &self.get_type(types).return_type;
        let scope = parent_scope.derive_fn(return_type);
        let scope = self.check_params(types, scope);
        let scope = match &self.body.value {
            FunctionBodyNode::Expression(expression) => {
                let (scope, _resolved_type) =
                    expression.check_expected(types, scope, Some(return_type));
                scope
            }
            FunctionBodyNode::Block(block) => {
                let (scope, _resolved_type) = block.check(types, scope);
                scope
            }
        };
        // TODO type check return type vs resolved type
        scope.parent()
    }

    fn check_params(&self, types: &TypeResolver, mut scope: Box<Scope>) -> Box<Scope> {
        let mut param_names = HashSet::new();
        for param in self.parameters.iter() {
            if param_names.contains(param.id()) {
                println!(
                    "Type error: Duplicate parameter named `{}` of function `{}`",
                    param.id(),
                    self.id()
                );
            } else {
                param_names.insert(param.id().clone());
                scope.add(param.id(), param.get_type(types).clone());
            }
        }
        scope
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
            .map_or(Type::Void, |return_type| return_type.get_type(types));

        FunctionType {
            parameters,
            return_type: Box::new(return_type),
        }
    }
}

impl Identified for FunctionNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
