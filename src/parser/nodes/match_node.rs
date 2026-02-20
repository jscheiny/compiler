use crate::{
    checker::{EnumType, Scope, Type, TypeResolver},
    parser::{ExpressionNode, MatchCaseNode, Node},
};

pub struct MatchNode {
    pub subject: Box<Node<ExpressionNode>>,
    pub cases: Vec<Node<MatchCaseNode>>,
}

impl MatchNode {
    pub fn check(
        &self,
        types: &TypeResolver,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
    ) -> (Box<Scope>, Type) {
        let (mut scope, subject_type) = self.check_subject(types, scope);
        let mut resolved_type = None;

        // TODO don't check for match branches in statement version
        // TODO completeness check

        for case in self.cases.iter() {
            let (new_scope, case_type) =
                case.check(types, scope, expected_type, subject_type.as_ref());
            scope = new_scope;

            // TODO dedupe with array parsing potentially
            if let Some(t) = resolved_type.as_ref() {
                if case_type.is_assignable_to(t, types) {
                    // Case type matches no error and keep going
                } else if t.is_assignable_to(&case_type, types) {
                    resolved_type = Some(case_type);
                } else {
                    println!(
                        "Type error: Mismatching types in match cases `{}` and `{}`",
                        t.format(types),
                        case_type.format(types)
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

    fn check_subject(
        &self,
        types: &TypeResolver,
        scope: Box<Scope>,
    ) -> (Box<Scope>, Option<EnumType>) {
        let (scope, subject_type) = self.subject.check(types, scope);
        let subject_type = subject_type.as_deref(types);
        if let Type::Enum(enum_type) = subject_type {
            (scope, Some(enum_type))
        } else {
            // TODO handle other types besides enums
            println!(
                "Type error: Match expression currently only supports enum types, found `{}`",
                subject_type.format(types)
            );
            (scope, None)
        }
    }
}
