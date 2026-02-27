use crate::{
    checker::{Scope, Type},
    parser::{ExpressionNode, Node, Operator, PrefixOperator, PrimitiveType},
};

pub struct PrefixOpExpressionNode {
    pub operator: Node<PrefixOperator>,
    pub expression: Box<Node<ExpressionNode>>,
}

impl PrefixOpExpressionNode {
    pub fn check(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        match *self.operator {
            PrefixOperator::LogicalNot => self.check_logical_not(scope),
            PrefixOperator::Negative => self.check_negative(scope),
        }
    }

    fn check_logical_not(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, resolved_type) = self.expression.check(scope);
        if !resolved_type.is_primitive(PrimitiveType::Bool, &scope.types) {
            scope.source.print_error(
                self.expression.span,
                &format!(
                    "Operand of `{}` should be of type `bool`",
                    self.operator.as_token(),
                ),
                &format!("found type: `{}`", resolved_type.format(&scope.types)),
            );
        }

        (scope, Type::Primitive(PrimitiveType::Bool))
    }

    fn check_negative(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, resolved_type) = self.expression.check(scope);
        if resolved_type.is_primitive(PrimitiveType::Float, &scope.types)
            || resolved_type.is_primitive(PrimitiveType::Int, &scope.types)
        {
            (scope, resolved_type)
        } else {
            if !matches!(resolved_type, Type::Error) {
                scope.source.print_error(
                    self.expression.span,
                    "Negation can only be applied to numeric types",
                    &format!("found type: `{}`", resolved_type.format(&scope.types)),
                );
            }
            (scope, Type::Error)
        }
    }
}
