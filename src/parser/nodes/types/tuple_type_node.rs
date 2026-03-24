use std::{cell::OnceCell, rc::Rc};

use crate::{
    checker::{Scope, Type, TypeParameterMap},
    parser::{Node, TypeNode},
};

pub struct TupleTypeNode {
    fields: Vec<Node<TypeNode>>,
    resolved_type: OnceCell<Type>,
}

impl TupleTypeNode {
    pub fn new(fields: Vec<Node<TypeNode>>) -> Self {
        Self {
            fields,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, scope: &Scope, type_params: Option<&TypeParameterMap>) -> Type {
        self.resolved_type
            .get_or_init(|| self.init_type(scope, type_params))
            .clone()
    }

    fn init_type(&self, scope: &Scope, type_params: Option<&TypeParameterMap>) -> Type {
        let fields = self
            .fields
            .iter()
            .map(|field| field.get_type(scope, type_params))
            .collect();

        Type::Tuple(Rc::new(fields))
    }
}
