use std::cell::OnceCell;

use crate::{
    checker::{FunctionType, TypeResolver},
    parser::{Node, NodeVec, TypeNode},
};

pub struct FunctionTypeNode {
    parameters: NodeVec<TypeNode>,
    return_type: Box<Node<TypeNode>>,
    resolved_type: OnceCell<FunctionType>,
}

impl FunctionTypeNode {
    pub fn new(parameters: NodeVec<TypeNode>, return_type: Box<Node<TypeNode>>) -> Self {
        Self {
            parameters,
            return_type,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self, types: &TypeResolver) {
        for parameter in self.parameters.iter() {
            parameter.check(types);
        }
        self.return_type.check(types);
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
            .collect();

        let return_type = Some(Box::new(self.return_type.get_type(types)));

        FunctionType {
            parameters,
            return_type,
        }
    }
}
