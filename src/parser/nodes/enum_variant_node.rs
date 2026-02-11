use std::cell::OnceCell;

use crate::{
    checker::{Type, TypeResolver},
    parser::{Identified, IdentifierNode, Node, TypeNode},
};

pub struct EnumVariantNode {
    identifier: Node<IdentifierNode>,
    type_def: Option<Node<TypeNode>>,
    resolved_type: OnceCell<Option<Type>>,
}

impl EnumVariantNode {
    pub fn new(identifier: Node<IdentifierNode>, type_def: Option<Node<TypeNode>>) -> Self {
        Self {
            identifier,
            type_def,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, types: &TypeResolver) -> Option<&Type> {
        self.resolved_type
            .get_or_init(|| self.get_type_impl(types))
            .as_ref()
    }

    fn get_type_impl(&self, types: &TypeResolver) -> Option<Type> {
        self.type_def.as_ref().map(|ty| ty.get_type(types))
    }
}

impl Identified for EnumVariantNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
