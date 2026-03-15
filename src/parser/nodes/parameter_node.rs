use std::cell::OnceCell;

use crate::{
    checker::{Scope, Type},
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

    pub fn get_type(&self, scope: &Scope) -> &Type {
        self.resolved_type.get_or_init(|| self.get_type_impl(scope))
    }

    fn get_type_impl(&self, scope: &Scope) -> Type {
        match self.type_def.as_ref() {
            Some(type_def) => type_def.get_type(scope, None),
            None => Type::Error,
        }
    }
}
