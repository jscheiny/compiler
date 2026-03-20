use std::{cell::OnceCell, rc::Rc};

use crate::{
    checker::{FunctionType, Scope, Type},
    parser::{NameNode, Node, NodeVec, ParameterNode, TypeNode},
};

pub struct FunctionSignatureNode {
    pub name: NameNode,
    pub parameters: NodeVec<ParameterNode>,
    pub return_type: Option<Node<TypeNode>>,
    resolved_type: OnceCell<Rc<FunctionType>>,
}

impl FunctionSignatureNode {
    pub fn new(
        name: NameNode,
        parameters: NodeVec<ParameterNode>,
        return_type: Option<Node<TypeNode>>,
    ) -> Self {
        Self {
            name,
            parameters,
            return_type,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, scope: &Scope) -> Rc<FunctionType> {
        self.resolved_type
            .get_or_init(|| self.init_type(scope))
            .clone()
    }

    fn init_type(&self, scope: &Scope) -> Rc<FunctionType> {
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
            .map_or(Type::Void, |return_type| return_type.get_type(scope, None));

        FunctionType::new(parameters, return_type)
    }
}
