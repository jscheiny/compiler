use std::collections::HashMap;

use crate::{
    checker::{Scope, Type},
    parser::{NameNode, TokenSpan, VariantMatchPatternNode},
};

pub enum MatchPatternNode {
    Variant(VariantMatchPatternNode),
    Binding(NameNode),
    Wildcard,
    Else,
}

impl MatchPatternNode {
    pub fn check(
        &self,
        scope: &Scope,
        span: TokenSpan,
        pattern_bindings: &mut HashMap<String, Type>,
        subject_type: &Type,
    ) {
        match self {
            Self::Variant(pattern) => pattern.check(scope, pattern_bindings, subject_type),
            Self::Binding(name) => {
                if pattern_bindings.contains_key(&name.value) {
                    scope.source.print_error(
                        span,
                        &format!("Duplicate pattern binding of `{name}`"),
                        "a binding of this name is declared elsewhere in this pattern",
                    );
                } else {
                    pattern_bindings.insert(name.value.clone(), subject_type.clone());
                }
            }
            Self::Wildcard | Self::Else => {}
        }
    }
}
