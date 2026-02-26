use std::cell::OnceCell;

use crate::{
    checker::{StructMember, StructMemberType, Type, TypeResolver},
    parser::{Identified, IdentifierNode, Node, TypeNode},
};

pub struct StructFieldNode {
    pub public: bool,
    pub identifier: Node<IdentifierNode>,
    pub type_def: Option<Node<TypeNode>>,
    resolved_type: OnceCell<Type>,
}

impl StructFieldNode {
    pub fn new(
        public: bool,
        identifier: Node<IdentifierNode>,
        type_def: Option<Node<TypeNode>>,
    ) -> Self {
        Self {
            public,
            identifier,
            type_def,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_member(&self, types: &TypeResolver) -> StructMember {
        StructMember {
            public: self.public,
            member_type: StructMemberType::Field(self.get_type(types).clone()),
        }
    }

    pub fn get_type(&self, types: &TypeResolver) -> &Type {
        self.resolved_type.get_or_init(|| self.resolve_type(types))
    }

    pub fn resolve_type(&self, types: &TypeResolver) -> Type {
        match self.type_def.as_ref() {
            Some(type_def) => type_def.get_type(types),
            None => Type::Error,
        }
    }
}

impl Identified for StructFieldNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
