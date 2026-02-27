use crate::{
    checker::{Scope, Type},
    parser::{ExpressionNode, Node, PrimitiveType},
};

pub struct IfExpressionNode {
    pub predicate: Box<Node<ExpressionNode>>,
    pub if_true: Box<Node<ExpressionNode>>,
    pub if_false: Box<Node<ExpressionNode>>,
}

impl IfExpressionNode {
    pub fn check(&self, scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        let (scope, predicate_type) = self.predicate.check(scope);
        if !predicate_type.is_primitive(PrimitiveType::Bool, &scope.types) {
            scope.source.print_error(
                self.predicate.span,
                "If expression predicate expected to be bool",
                &format!("found type: `{}`", predicate_type.format(&scope.types)),
            );
        }

        let (scope, true_type) = self.if_true.check_expected(scope, expected_type);
        let expected_type = expected_type.or(Some(&true_type));
        let (scope, false_type) = self.if_false.check_expected(scope, expected_type);

        if true_type.is_assignable_to(&false_type, &scope.types) {
            (scope, false_type)
        } else if false_type.is_assignable_to(&true_type, &scope.types) {
            (scope, true_type)
        } else {
            scope.source.print_error(
                self.if_false.span,
                "If expression branch types don't match",
                &format!(
                    "true branch type `{}` not compatible with false branch type `{}`",
                    true_type.format(&scope.types),
                    false_type.format(&scope.types),
                ),
            );
            (scope, true_type)
        }
    }
}
