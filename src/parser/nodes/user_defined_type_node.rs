use std::cell::OnceCell;

use crate::{
    checker::{Scope, Type},
    parser::{NameNode, Named},
};

pub struct UserDefinedTypeNode {
    pub name: NameNode,
    resolved_type: OnceCell<Type>,
}

impl UserDefinedTypeNode {
    pub fn new(name: NameNode) -> Self {
        Self {
            name,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, scope: &Scope) -> Type {
        self.resolved_type
            .get_or_init(|| self.get_type_impl(scope))
            .clone()
    }

    fn get_type_impl(&self, scope: &Scope) -> Type {
        let index = scope.get_type_index(self.name());
        if let Some(index) = index {
            return Type::Reference(index);
        }

        scope.source.print_error(
            self.name.span,
            &format!("Unknown type `{}`", self.name()),
            "could not find a type with this name",
        );
        Type::Error
    }
}

impl Named for UserDefinedTypeNode {
    fn name(&self) -> &String {
        &self.name
    }
}
