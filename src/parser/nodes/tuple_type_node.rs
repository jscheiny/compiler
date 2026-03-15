use std::{cell::OnceCell, rc::Rc};

use crate::{
    checker::{Scope, Type, TypeParameters},
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

    pub fn get_type(&self, scope: &Scope, type_params: Option<&TypeParameters>) -> Type {
        self.resolved_type
            .get_or_init(|| self.get_type_impl(scope, type_params))
            .clone()
    }

    fn get_type_impl(&self, scope: &Scope, type_params: Option<&TypeParameters>) -> Type {
        let fields = self
            .fields
            .iter()
            .map(|field| field.get_type(scope, type_params))
            .collect();

        Type::Tuple(Rc::new(fields))
    }
}
