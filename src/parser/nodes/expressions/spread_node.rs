use std::rc::Rc;

use crate::{
    checker::{Scope, Type},
    parser::{ExpressionNode, Node},
};

pub struct SpreadNode {
    pub expression: Box<Node<ExpressionNode>>,
}

impl SpreadNode {
    pub fn check_valid(
        &self,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
    ) -> (Box<Scope>, Rc<Vec<Type>>) {
        let (scope, result_type) = self.expression.check_expected(scope, expected_type);
        if let Type::Tuple(types) = result_type {
            (scope, types)
        } else {
            self.print_non_tuple_error(&scope, &result_type);
            let types = Rc::new(vec![result_type]);
            (scope, types)
        }
    }

    pub fn check_invalid(
        &self,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
    ) -> (Box<Scope>, Type) {
        scope.source.print_error(
            self.expression.span.before(),
            "Unexpected spread expression",
            "spread expression must be an argument or tuple element",
        );

        let (scope, result_type) = self.expression.check_expected(scope, expected_type);
        if !matches!(result_type, Type::Tuple(_)) && !result_type.is_error() {
            self.print_non_tuple_error(&scope, &result_type);
        }

        (scope, result_type)
    }

    fn print_non_tuple_error(&self, scope: &Scope, found_type: &Type) {
        scope.source.print_error(
            self.expression.span,
            "Spread expression should be a tuple",
            &format!("found type `{}`", found_type.format(scope)),
        );
    }
}
