use std::cell::OnceCell;

use crate::{
    checker::{Type, TypeResolver},
    parser::{Identified, IdentifierNode, ParseNode, TypeNode},
};

pub struct ParameterNode {
    identifier: ParseNode<IdentifierNode>,
    type_def: Option<ParseNode<TypeNode>>,
    resolved_type: OnceCell<Type>,
}

impl ParameterNode {
    pub fn new(
        identifier: ParseNode<IdentifierNode>,
        type_def: Option<ParseNode<TypeNode>>,
    ) -> Self {
        Self {
            identifier,
            type_def,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self, types: &TypeResolver) {
        if let Some(type_def) = self.type_def.as_ref() {
            type_def.check(types);
        }
    }

    pub fn get_type(&self, types: &TypeResolver) -> &Type {
        self.resolved_type.get_or_init(|| self.get_type_impl(types))
    }

    fn get_type_impl(&self, types: &TypeResolver) -> Type {
        match self.type_def.as_ref() {
            Some(type_def) => type_def.get_type(types),
            None => Type::Error,
        }
    }
}

impl Identified for ParameterNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
