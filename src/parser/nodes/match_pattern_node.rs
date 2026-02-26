use std::collections::HashMap;

use crate::{
    checker::{Scope, Type},
    parser::{Identified, IdentifierNode, Node, TokenSpan},
};

pub enum MatchPatternNode {
    Variant(VariantMatchPattern),
    Binding(IdentifierNode),
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
            MatchPatternNode::Binding(identifier) => {
                if bindings.contains_key(&identifier.0) {
                    scope.source.print_type_error(
                        span,
                        &format!("Duplicate pattern binding of `{}`", identifier.0),
                        "a binding of this name is declared elsewhere in this pattern",
                    );
                } else {
                    bindings.insert(identifier.0.clone(), subject_type.clone());
                }
            }
            MatchPatternNode::Else => {}
        }
    }
}

pub struct VariantMatchPattern {
    pub identifier: Node<IdentifierNode>,
    pub inner_pattern: Option<Box<Node<MatchPatternNode>>>,
}

impl VariantMatchPattern {
    pub fn check(&self, scope: &Scope, bindings: &mut HashMap<String, Type>, subject_type: &Type) {
        if let Type::Enum(enum_type) = subject_type.deref(&scope.types) {
            if let Some(variant) = enum_type.variants.get(self.identifier.id()) {
                if let Some(inner_type) = variant {
                    if self.inner_pattern.is_none() {
                        // TODO consider relaxing this when the subject is just an identifier...
                        scope.source.print_type_error(
                            self.identifier.span,
                            "Expected binding pattern",
                            &format!(
                                "typed variant `{}` must have a binding pattern",
                                self.identifier.id()
                            ),
                        );
                    } else {
                        return self.check_inner_pattern(scope, bindings, inner_type);
                    }
                } else if let Some(inner_pattern) = self.inner_pattern.as_ref() {
                    scope.source.print_type_error(
                        inner_pattern.span,
                        "Unexpected binding pattern",
                        &format!(
                            "untyped variant `{}` cannot use a binding pattern",
                            self.identifier.id()
                        ),
                    );
                }
            } else {
                scope.source.print_type_error(
                    self.identifier.span,
                    &format!("Could not find variant `{}`", self.identifier.id()),
                    &format!("enum `{}` has no such variant", enum_type.identifier),
                );
            }
        } else if !matches!(subject_type, Type::Error) {
            scope.source.print_type_error(
                self.identifier.span,
                "Unexpected variant pattern",
                &format!(
                    "cannot use variant pattern on non-enum type `{}`",
                    subject_type.format(&scope.types),
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
