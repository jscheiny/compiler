use std::collections::HashMap;

use crate::{
    checker::{Scope, Type},
    parser::{NameNode, Node, TokenSpan},
};

pub enum MatchPatternNode {
    Variant(VariantMatchPattern),
    Binding(NameNode),
    Else,
}

impl MatchPatternNode {
    pub fn check(
        &self,
        scope: &Scope,
        span: TokenSpan,
        bindings: &mut HashMap<String, Type>,
        subject_type: &Type,
    ) {
        match self {
            MatchPatternNode::Variant(pattern) => pattern.check(scope, bindings, subject_type),
            MatchPatternNode::Binding(name) => {
                if bindings.contains_key(&name.value) {
                    scope.source.print_error(
                        span,
                        &format!("Duplicate pattern binding of `{name}`"),
                        "a binding of this name is declared elsewhere in this pattern",
                    );
                } else {
                    bindings.insert(name.value.clone(), subject_type.clone());
                }
            }
            MatchPatternNode::Else => {}
        }
    }
}

pub struct VariantMatchPattern {
    pub name: NameNode,
    pub inner_pattern: Option<Box<Node<MatchPatternNode>>>,
}

impl VariantMatchPattern {
    pub fn check(&self, scope: &Scope, bindings: &mut HashMap<String, Type>, subject_type: &Type) {
        if let Type::Enum(enum_type) = subject_type {
            if let Some(variant) = enum_type.variants.get(&self.name.value) {
                if let Some(inner_type) = variant {
                    if self.inner_pattern.is_none() {
                        // TODO consider relaxing this when the subject is just a name...
                        scope.source.print_error(
                            self.name.span,
                            "Expected binding pattern",
                            &format!("typed variant `{}` must have a binding pattern", self.name),
                        );
                    } else {
                        return self.check_inner_pattern(scope, bindings, inner_type);
                    }
                } else if let Some(inner_pattern) = self.inner_pattern.as_ref() {
                    scope.source.print_error(
                        inner_pattern.span,
                        "Unexpected binding pattern",
                        &format!(
                            "untyped variant `{}` cannot use a binding pattern",
                            self.name
                        ),
                    );
                }
            } else {
                scope.source.print_error(
                    self.name.span,
                    &format!("Could not find variant `{}`", self.name),
                    &format!("enum `{}` has no such variant", enum_type.name()),
                );
            }
        } else if !subject_type.is_error() {
            scope.source.print_error(
                self.name.span,
                "Unexpected variant pattern",
                &format!(
                    "cannot use variant pattern on non-enum type `{}`",
                    subject_type,
                ),
            );
        }
        self.check_inner_pattern(scope, bindings, &Type::Error);
    }

    fn check_inner_pattern(
        &self,
        scope: &Scope,
        bindings: &mut HashMap<String, Type>,
        bound_type: &Type,
    ) {
        if let Some(inner_pattern) = self.inner_pattern.as_ref() {
            inner_pattern.check(scope, inner_pattern.span, bindings, bound_type);
        }
    }
}
