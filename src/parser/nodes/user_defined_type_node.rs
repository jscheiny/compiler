use std::cell::OnceCell;

use crate::{
    checker::{Scope, Type},
    parser::{Identified, NameNode, Node},
};

pub struct UserDefinedTypeNode {
    pub identifier: Node<NameNode>,
    resolved_type: OnceCell<Type>,
}

impl UserDefinedTypeNode {
    pub fn new(identifier: Node<NameNode>) -> Self {
        Self {
            identifier,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, scope: &Scope) -> Type {
        self.resolved_type
            .get_or_init(|| self.get_type_impl(scope))
            .clone()
    }

    fn get_type_impl(&self, scope: &Scope) -> Type {
        let index = scope.get_type_index(self.id());
        if let Some(index) = index {
            return Type::Reference(index);
        }

        scope.source.print_error(
            self.identifier.span,
            &format!("Unknown type `{}`", self.id()),
            "could not find a type with this name",
        );
        Type::Error
    }
}

impl Identified for UserDefinedTypeNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
