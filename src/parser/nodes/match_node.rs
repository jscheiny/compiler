use crate::{
    checker::{Scope, Type},
    parser::{ExpressionNode, MatchCaseNode, Node},
};

pub struct MatchNode {
    pub subject: Box<Node<ExpressionNode>>,
    pub cases: Vec<Node<MatchCaseNode>>,
}

impl MatchNode {
    pub fn check_statement(&self, scope: Box<Scope>) -> Box<Scope> {
        let (mut scope, subject_type) = self.check_subject(scope);
        for case in self.cases.iter() {
            let (new_scope, _) = case.check(scope, None, &subject_type);
            scope = new_scope
        }

        scope
    }

    pub fn check(&self, scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        let (mut scope, subject_type) = self.check_subject(scope);
        let mut resolved_type = None;

        // TODO don't check for match branches in statement version
        // TODO completeness check

        for case in self.cases.iter() {
            let (new_scope, case_type) = case.check(scope, expected_type, &subject_type);
            scope = new_scope;

            // TODO dedupe with array parsing potentially
            if let Some(t) = resolved_type.as_ref() {
                if case_type.is_assignable_to(t, &scope.types) {
                    // Case type matches: no error and keep going
                } else if t.is_assignable_to(&case_type, &scope.types) {
                    resolved_type = Some(case_type);
                } else {
                    scope.source.print_type_error(
                        case.if_match.span,
                        "Match cases types don't match",
                        &format!(
                            "case results in type `{}` which does not match previous type `{}`",
                            case_type.format(&scope.types),
                            t.format(&scope.types)
                        ),
                    );
                }
            } else {
                resolved_type = Some(case_type);
            }
        }

        if let Some(resolved_type) = resolved_type {
            (scope, resolved_type)
        } else {
            (scope, Type::Void)
        }
    }

    fn check_subject(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, subject_type) = self.subject.check(scope);
        let subject_type = subject_type.as_deref(&scope.types);
        if !matches!(subject_type, Type::Enum(_)) && !matches!(subject_type, Type::Error) {
            // TODO handle other types besides enums
            scope.source.print_type_error(
                self.subject.span,
                "Match expressions only support enums",
                &format!("found type: `{}`", subject_type.format(&scope.types)),
            );
        }

        (scope, subject_type)
    }
}
