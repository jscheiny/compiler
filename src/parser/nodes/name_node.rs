use std::fmt::Display;

use crate::{
    checker::{Scope, Type},
    parser::Node,
};

pub type NameNode = Node<String>;

impl NameNode {
    pub fn check(&self, scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        let expected_enum_type = expected_type.and_then(|e| match e.deref(&scope) {
            Type::Enum(enum_type) => Some(enum_type),
            _ => None,
        });

        let type_value = scope.get_type_index(self).map(Type::Reference);
        if let Some(resolved_type) = scope.get_value(self) {
            (scope, resolved_type)
        } else if let Some(type_value) = type_value {
            scope.source.print_error(
                self.span,
                "Types cannot be used as values",
                &format!("cannot use type `{}` as a value", type_value.format(&scope)),
            );
            (scope, Type::Error)
        } else if let Some(enum_type) = expected_enum_type {
            if let Some(variant_type) = enum_type.get_variant(self) {
                (scope, variant_type)
            } else {
                scope.source.print_error(
                    self.span,
                    &format!("Could not find value `{self}`"),
                    "no such symbol found",
                );
                (scope, Type::Error)
            }
        } else {
            scope.source.print_error(
                self.span,
                &format!("Could not find value `{self}`"),
                "no such symbol found",
            );
            (scope, Type::Error)
        }
    }
}

impl Display for NameNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
