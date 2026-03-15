use std::{cell::OnceCell, rc::Rc};

use crate::{
    checker::{Generics, Scope, Type},
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

    pub fn get_type(&self, scope: &Scope, generics: Generics<'_>) -> Type {
        self.resolved_type
            .get_or_init(|| self.get_type_impl(scope, generics))
            .clone()
    }

    fn get_type_impl(&self, scope: &Scope, generics: Generics<'_>) -> Type {
        let fields = self
            .fields
            .iter()
            .map(|field| field.get_type(scope, generics))
            .collect();

        Type::Tuple(Rc::new(fields))
    }
}
