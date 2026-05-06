use std::cell::OnceCell;

use crate::{
    checker::{Type, Types},
    parser::{NameNode, Node, TypeNode, get_maybe_spread_type},
};

pub struct ParameterNode {
    pub is_spread: bool,
    pub name: NameNode,
    pub type_def: Option<Node<TypeNode>>,
    resolved_type: OnceCell<Type>,
}

impl ParameterNode {
    pub fn new(is_spread: bool, name: NameNode, type_def: Option<Node<TypeNode>>) -> Self {
        Self {
            is_spread,
            name,
            type_def,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_call_type(&self, types: &impl Types) -> Vec<Type> {
        let body_type = self.get_body_type(types);
        let type_span = self.type_def.as_ref().map(|t| t.span);
        get_maybe_spread_type(types, self.is_spread, body_type.clone(), type_span)
    }

    pub fn get_body_type(&self, types: &impl Types) -> &Type {
        self.resolved_type.get_or_init(|| self.init_type(types))
    }

    fn init_type(&self, types: &impl Types) -> Type {
        match self.type_def.as_ref() {
            Some(type_def) => type_def.get_type(types, None),
            None => Type::Error,
        }
    }
}
