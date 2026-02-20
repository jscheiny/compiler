use std::collections::HashMap;

use crate::{
    checker::{Type, TypeResolver},
    parser::{Identified, IdentifierNode, Node},
};

pub enum MatchPatternNode {
    Variant(VariantMatchPattern),
    Binding(IdentifierNode),
}

impl MatchPatternNode {
    pub fn check(
        &self,
        types: &TypeResolver,
        bindings: &mut HashMap<String, Type>,
        subject_type: &Type,
    ) {
        match self {
            MatchPatternNode::Variant(pattern) => pattern.check(types, bindings, subject_type),
            MatchPatternNode::Binding(identifier) => {
                if bindings.contains_key(&identifier.0) {
                    println!(
                        "Type error: Duplicate binding for identifier `{}`",
                        identifier.0
                    );
                } else {
                    bindings.insert(identifier.0.clone(), subject_type.clone());
                }
            }
        }
    }
}

pub struct VariantMatchPattern {
    pub identifier: Node<IdentifierNode>,
    pub inner_pattern: Option<Box<Node<MatchPatternNode>>>,
}

impl VariantMatchPattern {
    pub fn check(
        &self,
        types: &TypeResolver,
        bindings: &mut HashMap<String, Type>,
        subject_type: &Type,
    ) {
        if let Type::Enum(enum_type) = subject_type.deref(types) {
            if let Some(variant) = enum_type.variants.get(self.identifier.id()) {
                if let Some(inner_type) = variant {
                    if self.inner_pattern.is_none() {
                        println!(
                            "Type error: Expected binding pattern for typed variant `{}`",
                            self.identifier.id()
                        );
                    } else {
                        return self.check_inner_pattern(types, bindings, inner_type);
                    }
                } else {
                    if self.inner_pattern.is_some() {
                        println!(
                            "Type error: Expected no binding pattern for untyped variant `{}`",
                            self.identifier.id()
                        );
                    }
                }
            } else {
                println!(
                    "Type error: No such variant `{}` on enum `{}`",
                    self.identifier.id(),
                    enum_type.identifier
                );
            }
        } else if !matches!(subject_type, Type::Error) {
            println!(
                "Type error: Cannot use variant pattern on non-enum type `{}`",
                subject_type.format(types),
            );
        }
        self.check_inner_pattern(types, bindings, &Type::Error);
    }

    fn check_inner_pattern(
        &self,
        types: &TypeResolver,
        bindings: &mut HashMap<String, Type>,
        bound_type: &Type,
    ) {
        if let Some(inner_pattern) = self.inner_pattern.as_ref() {
            inner_pattern.check(types, bindings, bound_type);
        }
    }
}
