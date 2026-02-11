use std::cell::OnceCell;

use crate::{
    checker::{Type, TypeResolver},
    parser::{Identified, IdentifierNode, ParseNode, TypeNode},
};

pub struct TypeAliasNode {
    identifier: ParseNode<IdentifierNode>,
    type_def: ParseNode<TypeNode>,
    resolved_type: OnceCell<Type>,
}

impl TypeAliasNode {
    pub fn new(identifier: ParseNode<IdentifierNode>, type_def: ParseNode<TypeNode>) -> Self {
        Self {
            identifier,
            type_def,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, types: &TypeResolver) -> &Type {
        self.resolved_type
            .get_or_init(|| self.type_def.get_type(types))
    }
}

impl Identified for TypeAliasNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
