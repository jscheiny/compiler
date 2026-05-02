use std::cell::OnceCell;

use crate::{
    checker::{Type, Types},
    parser::{NameNode, Node, TypeNode},
};

pub struct ParameterNode {
    pub name: NameNode,
    pub type_def: Option<Node<TypeNode>>,
    resolved_type: OnceCell<Type>,
}

impl ParameterNode {
    pub fn new(name: NameNode, type_def: Option<Node<TypeNode>>) -> Self {
        Self {
            name,
            type_def,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, types: &impl Types) -> &Type {
        self.resolved_type.get_or_init(|| self.init_type(types))
    }

    fn init_type(&self, types: &impl Types) -> Type {
        match self.type_def.as_ref() {
            Some(type_def) => type_def.get_type(types, None, None),
            None => Type::Error,
        }
    }
}
