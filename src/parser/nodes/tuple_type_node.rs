use std::cell::OnceCell;

use crate::{
    checker::{Type, TypeResolver},
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

    pub fn get_type(&self, types: &TypeResolver) -> &Type {
        self.resolved_type.get_or_init(|| self.resolve_type(types))
    }

    fn resolve_type(&self, types: &TypeResolver) -> Type {
        let fields = self
            .fields
            .iter()
            .map(|field| field.get_type(types))
            .collect();

        Type::Tuple(fields)
    }
}
