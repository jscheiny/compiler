use std::collections::HashMap;

use crate::{
    checker::{Scope, Type},
    parser::{MatchPatternNode, Node, TokenSpan},
};

pub struct TupleMatchPatternNode {
    pub inner_patterns: Vec<Node<MatchPatternNode>>,
}

impl TupleMatchPatternNode {
    pub fn check(
        &self,
        scope: &Scope,
        span: TokenSpan,
        pattern_bindings: &mut HashMap<String, Type>,
        subject_type: &Type,
    ) {
        let tuple_type = match subject_type {
            Type::Tuple(tuple_type) => Some(tuple_type),
            Type::Error => None,
            _ => {
                scope.source.print_error(
                    span,
                    "Unexpected variant pattern",
                    &format!("cannot use tuple pattern on non-tuple type `{subject_type}`"),
                );
                None
            }
        };

        if let Some(tuple_type) = tuple_type
            && self.inner_patterns.len() != tuple_type.len()
        {
            scope.source.print_error(
                span,
                "Incorrect number of patterns to match type",
                &format!(
                    "expected {} patterns to match type `{subject_type}`",
                    tuple_type.len()
                ),
            );
        }

        for (index, inner_pattern) in self.inner_patterns.iter().enumerate() {
            let inner_type = tuple_type
                .and_then(|types| types.get(index))
                .unwrap_or(&Type::Error);
            inner_pattern.check(scope, inner_pattern.span, pattern_bindings, inner_type);
        }
    }
}
