use std::cell::OnceCell;

use crate::{
    checker::{Scope, Type},
    parser::{NameNode, Node, TypeNode},
};

pub struct EnumVariantNode {
    pub name: NameNode,
    pub type_def: Option<Node<TypeNode>>,
    resolved_type: OnceCell<Option<Type>>,
}

impl EnumVariantNode {
    pub fn new(name: NameNode, type_def: Option<Node<TypeNode>>) -> Self {
        Self {
            name,
            type_def,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, scope: &Scope) -> Option<&Type> {
        self.resolved_type
            .get_or_init(|| self.init_type(scope))
            .as_ref()
    }

    fn init_type(&self, scope: &Scope) -> Option<Type> {
        self.type_def
            .as_ref()
            .map(|ty| ty.get_type(scope, None, None))
    }
}
