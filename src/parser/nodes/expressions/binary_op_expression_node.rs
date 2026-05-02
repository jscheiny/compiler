use crate::{
    checker::{Scope, Type},
    parser::{BinaryOperator, ExpressionNode, Node, Operator, PrimitiveType, TokenSpan},
};

pub struct BinaryOpExpressionNode {
    pub left: Box<Node<ExpressionNode>>,
    pub operator: Node<BinaryOperator>,
    pub right: Box<Node<ExpressionNode>>,
}

impl BinaryOpExpressionNode {
    pub fn check(&self, scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        use BinaryOperator as O;
        match *self.operator {
            O::Add
            | O::AddAssign
            | O::Subtract
            | O::SubtractAssign
            | O::Multiply
            | O::MultiplyAssign
            | O::Divide
            | O::DivideAssign
            | O::Mod
            | O::ModAssign
            | O::Assign
            | O::Equal
            | O::NotEqual
            | O::LessThan
            | O::LessThanOrEqual
            | O::GreaterThan
            | O::GreaterThanOrEqual => todo_unimplemented_operator(scope),
            O::FunctionApplication => self.check_function_application(scope, expected_type),
            // TODO can we remove this panic somehow?
            O::Comma => panic!("ERROR: How did we get here?"),
            O::LogicalAnd | O::LogicalOr => self.check_logical_op(scope),
        }
    }

    fn check_function_application(
        &self,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
    ) -> (Box<Scope>, Type) {
        let (scope, left_type) = self.left.check(scope);
        // TODO the expected type here should be a function from left_type -> expected_type
        let (scope, right_type) = self.right.check_expected(scope, expected_type);
        let function_type = right_type.to_function(&scope);

        if let Some(function_type) = function_type {
            if function_type.parameters.len() != 1 {
                scope.source.print_error(
                    self.right.span,
                    "Applied function must take only one parameter",
                    &format!("type: `{}`", Type::Function(function_type.clone())),
                );
            }

            if !function_type.parameters.is_empty()
                && !left_type.is_assignable_to(&function_type.parameters[0], &scope)
            {
                scope.source.print_error(
                    self.left.span,
                    "Function application argument does not match parameter type",
                    &format!(
                        "expected value of type `{left_type}`, found `{}`",
                        function_type.parameters[0]
                    ),
                );
            }

            (scope, *function_type.return_type.clone())
        } else {
            if !right_type.is_error() {
                scope.source.print_error(
                    self.right.span,
                    "Cannot apply function",
                    &format!("type `{right_type}` is not callable"),
                );
            }
            (scope, Type::Error)
        }
    }

    fn check_logical_op(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let bool_type = Type::Primitive(PrimitiveType::Bool);

        let (scope, left_type) = self.left.check(scope);
        if !left_type.is_primitive(PrimitiveType::Bool, &scope) {
            self.print_operand_error(&scope, self.left.span, &bool_type, &left_type);
        }

        let (scope, right_type) = self.right.check(scope);
        if !right_type.is_primitive(PrimitiveType::Bool, &scope) {
            self.print_operand_error(&scope, self.right.span, &bool_type, &right_type);
        }

        (scope, Type::Primitive(PrimitiveType::Bool))
    }

    fn print_operand_error(
        &self,
        scope: &Scope,
        span: TokenSpan,
        expected_type: &Type,
        found_type: &Type,
    ) {
        scope.source.print_error(
            span,
            &format!(
                "Operands of `{}` should be of type `{expected_type}`",
                self.operator.as_token(),
            ),
            &format!("found type: `{found_type}`"),
        );
    }
}

fn todo_unimplemented_operator(scope: Box<Scope>) -> (Box<Scope>, Type) {
    (scope, Type::Error)
}
