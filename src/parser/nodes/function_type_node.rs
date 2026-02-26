use std::cell::OnceCell;

use crate::{
    checker::{FunctionType, TypeResolver},
    lexer::SourceCode,
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

    pub fn get_type(&self, types: &TypeResolver, source: &SourceCode) -> &FunctionType {
        self.resolved_type
            .get_or_init(|| self.get_type_impl(types, source))
    }

    fn get_type_impl(&self, types: &TypeResolver, source: &SourceCode) -> FunctionType {
        let parameters = self
            .parameters
            .value
            .iter()
            .map(|parameter| parameter.get_type(types, source))
            .collect();

        let return_type = Box::new(self.return_type.get_type(types, source));

        FunctionType {
            parameters,
            return_type,
        }
    }
}
