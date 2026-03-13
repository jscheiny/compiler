use std::cell::OnceCell;

use crate::{
    checker::{Scope, Type},
    parser::{NameNode, Named, Node, TypeNode},
};

pub struct EnumVariantNode {
    pub identifier: Node<NameNode>,
    pub type_def: Option<Node<TypeNode>>,
    resolved_type: OnceCell<Option<Type>>,
}

impl EnumVariantNode {
    pub fn new(identifier: Node<NameNode>, type_def: Option<Node<TypeNode>>) -> Self {
        Self {
            identifier,
            type_def,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, scope: &Scope) -> Option<&Type> {
        self.resolved_type
            .get_or_init(|| self.get_type_impl(scope))
            .as_ref()
    }

    fn get_type_impl(&self, scope: &Scope) -> Option<Type> {
        self.type_def.as_ref().map(|ty| ty.get_type(scope))
    }
}

impl Named for EnumVariantNode {
    fn name(&self) -> &String {
        self.identifier.name()
    }
}
