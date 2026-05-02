use std::cell::OnceCell;

use crate::{
    checker::{Type, Types},
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

    pub fn get_type(&self, types: &impl Types) -> Option<&Type> {
        self.resolved_type
            .get_or_init(|| self.init_type(types))
            .as_ref()
    }

    fn init_type(&self, types: &impl Types) -> Option<Type> {
        self.type_def
            .as_ref()
            .map(|ty| ty.get_type(types, None, None))
    }
}
