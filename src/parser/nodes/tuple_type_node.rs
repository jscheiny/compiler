use std::cell::OnceCell;

use crate::{
    checker::{Type, TypeResolver},
    lexer::SourceCode,
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

    pub fn get_type(&self, types: &TypeResolver, source: &SourceCode) -> &Type {
        self.resolved_type
            .get_or_init(|| self.resolve_type(types, source))
    }

    fn resolve_type(&self, types: &TypeResolver, source: &SourceCode) -> Type {
        let fields = self
            .fields
            .iter()
            .map(|field| field.get_type(types, source))
            .collect();

        Type::Tuple(fields)
    }
}
