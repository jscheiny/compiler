use std::cell::OnceCell;

use crate::{
    checker::{Scope, Type},
    parser::{Identified, IdentifierNode, Node, TypeNode},
};

pub struct TypeAliasNode {
    pub identifier: Node<IdentifierNode>,
    pub type_def: Node<TypeNode>,
    resolved_type: OnceCell<Type>,
}

impl TypeAliasNode {
    pub fn new(identifier: Node<IdentifierNode>, type_def: Node<TypeNode>) -> Self {
        Self {
            identifier,
            type_def,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self) {
        // TODO check for recursion
    }

    pub fn get_type(&self, scope: &Scope) -> &Type {
        self.resolved_type
            .get_or_init(|| self.type_def.get_type(scope))
    }
}

impl Identified for TypeAliasNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
