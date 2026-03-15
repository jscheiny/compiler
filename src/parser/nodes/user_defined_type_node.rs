use std::cell::OnceCell;

use crate::{
    checker::{Scope, Type, TypeParameters},
    parser::NameNode,
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

    pub fn get_type(&self, scope: &Scope, type_parameters: Option<&TypeParameters>) -> Type {
        self.resolved_type
            .get_or_init(|| self.init_type(scope, type_parameters))
            .clone()
    }

    fn init_type(&self, scope: &Scope, type_parameters: Option<&TypeParameters>) -> Type {
        let type_parameter = type_parameters.and_then(|t| t.get(&self.name.value));
        if let Some(type_parameter) = type_parameter {
            return Type::TypeParameter(type_parameter.clone());
        }

        let index = scope.get_type_index(&self.name);
        if let Some(index) = index {
            return Type::Reference(index);
        }

        scope.source.print_error(
            self.name.span,
            &format!("Unknown type `{}`", self.name),
            "could not find a type with this name",
        );
        Type::Error
    }
}
