use std::cell::OnceCell;

use crate::{
    checker::{Scope, StructMember, StructMemberType, Type},
    parser::{NameNode, Node, TypeNode},
};

pub struct StructFieldNode {
    pub public: bool,
    pub name: NameNode,
    pub type_def: Option<Node<TypeNode>>,
    resolved_type: OnceCell<Type>,
}

impl StructFieldNode {
    pub fn new(public: bool, name: NameNode, type_def: Option<Node<TypeNode>>) -> Self {
        Self {
            public,
            name,
            type_def,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_member(&self, scope: &Scope) -> StructMember {
        StructMember {
            public: self.public,
            member_type: StructMemberType::Field(self.get_type(scope).clone()),
        }
    }

    pub fn get_type(&self, scope: &Scope) -> &Type {
        self.resolved_type.get_or_init(|| self.init_type(scope))
    }

    fn init_type(&self, scope: &Scope) -> Type {
        match self.type_def.as_ref() {
            Some(type_def) => type_def.get_type(scope, None),
            None => Type::Error,
        }
    }
}
