use std::{cell::OnceCell, rc::Rc};

use crate::{
    checker::{FunctionType, Scope, TypeParameterMap},
    parser::{Node, TypeListNode, TypeNode, VisitedTypes},
};

pub struct FunctionTypeNode {
    parameters: TypeListNode,
    return_type: Box<Node<TypeNode>>,
    resolved_type: OnceCell<Rc<FunctionType>>,
}

impl FunctionTypeNode {
    pub fn new(parameters: TypeListNode, return_type: Box<Node<TypeNode>>) -> Self {
        Self {
            parameters,
            return_type,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(
        &self,
        scope: &Scope,
        type_params: Option<&TypeParameterMap>,
        visited: VisitedTypes,
    ) -> Rc<FunctionType> {
        self.resolved_type
            .get_or_init(|| self.init_type(scope, type_params, visited))
            .clone()
    }

    fn init_type(
        &self,
        scope: &Scope,
        type_params: Option<&TypeParameterMap>,
        visited: VisitedTypes,
    ) -> Rc<FunctionType> {
        let parameters = self
            .parameters
            .get_type(scope, type_params, visited.clone());
        let return_type = self.return_type.get_type(scope, type_params, visited);
        FunctionType::new(parameters, return_type)
    }
}
