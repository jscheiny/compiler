use std::{cell::OnceCell, rc::Rc};

use crate::{
    checker::{FunctionType, Scope, TypeParameters},
    parser::{Node, NodeVec, TypeNode},
};

pub struct FunctionTypeNode {
    parameters: NodeVec<TypeNode>,
    return_type: Box<Node<TypeNode>>,
    resolved_type: OnceCell<Rc<FunctionType>>,
}

impl FunctionTypeNode {
    pub fn new(parameters: NodeVec<TypeNode>, return_type: Box<Node<TypeNode>>) -> Self {
        Self {
            parameters,
            return_type,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(
        &self,
        scope: &Scope,
        type_params: Option<&TypeParameters>,
    ) -> Rc<FunctionType> {
        self.resolved_type
            .get_or_init(|| self.get_type_impl(scope, type_params))
            .clone()
    }

    fn get_type_impl(
        &self,
        scope: &Scope,
        type_params: Option<&TypeParameters>,
    ) -> Rc<FunctionType> {
        let parameters = self
            .parameters
            .value
            .iter()
            .map(|parameter| parameter.get_type(scope, type_params))
            .collect();

        let return_type = Box::new(self.return_type.get_type(scope, type_params));

        Rc::new(FunctionType {
            parameters,
            return_type,
        })
    }
}
