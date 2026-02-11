use std::cell::OnceCell;

use crate::{
    checker::{Type, TypeResolver},
    parser::{ParseNode, TypeParseNode},
};

pub struct TupleTypeParseNode {
    fields: Vec<ParseNode<TypeParseNode>>,
    resolved_type: OnceCell<Type>,
}

impl TupleTypeParseNode {
    pub fn new(fields: Vec<ParseNode<TypeParseNode>>) -> Self {
        Self {
            fields,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self, types: &TypeResolver) {
        for field in self.fields.iter() {
            field.check(types);
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
