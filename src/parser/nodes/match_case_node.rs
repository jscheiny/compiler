use std::collections::HashMap;

use crate::{
    checker::{Scope, ScopeType, Type},
    parser::{ExpressionNode, MatchPatternNode, Node},
};

pub struct MatchCaseNode {
    pub pattern: Node<MatchPatternNode>,
    pub if_match: Node<ExpressionNode>,
}

impl MatchCaseNode {
    pub fn check(
        &self,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
        subject_type: &Type,
    ) -> (Box<Scope>, Type) {
        let mut bindings = HashMap::new();
        self.pattern
            .check(&scope, self.pattern.span, &mut bindings, subject_type);
        scope.nest_with(ScopeType::MatchCase, |mut scope| {
            for (identifier, bound_type) in bindings {
                scope.add(identifier.as_str(), bound_type);
            }
            // TODO handle pattern checking
            let (scope, resolved_type) = self.if_match.check_expected(scope, expected_type);
            (scope, resolved_type)
        })
    }
}
