use std::cell::OnceCell;

use crate::{
    checker::{Scope, Type},
    parser::{NameNode, Named, Node, TypeNode},
};

pub struct TypeAliasNode {
    pub name: Node<NameNode>,
    pub type_def: Node<TypeNode>,
    resolved_type: OnceCell<Type>,
}

impl TypeAliasNode {
    pub fn new(name: Node<NameNode>, type_def: Node<TypeNode>) -> Self {
        Self {
            name,
            type_def,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check_statement(&self, mut scope: Box<Scope>) -> Box<Scope> {
        scope.add_type(self.name.name(), self.get_type(&scope).clone());
        scope
        // TODO check for recursion
    }

    pub fn get_type(&self, scope: &Scope) -> &Type {
        self.resolved_type
            .get_or_init(|| self.type_def.get_type(scope))
    }
}

impl Named for TypeAliasNode {
    fn name(&self) -> &String {
        self.name.name()
    }
}
