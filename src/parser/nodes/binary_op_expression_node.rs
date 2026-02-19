use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{BinaryOperator, ExpressionNode, Node, PrimitiveType},
};

pub struct BinaryOpExpressionNode {
    pub left: Box<Node<ExpressionNode>>,
    pub operator: Node<BinaryOperator>,
    pub right: Box<Node<ExpressionNode>>,
}

impl BinaryOpExpressionNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        use BinaryOperator as O;
        match *self.operator {
            O::Add => todo!("Implement type checking for +"),
            O::AddAssign => todo!("Implement type checking for +="),
            O::Subtract => todo!("Implement type checking for -"),
            O::SubtractAssign => todo!("Implement type checking for -="),
            O::Multiply => todo!("Implement type checking for *"),
            O::MultiplyAssign => todo!("Implement type checking for *="),
            O::Divide => todo!("Implement type checking for /"),
            O::DivideAssign => todo!("Implement type checking for /="),
            O::Mod => todo!("Implement type checking for %"),
            O::ModAssign => todo!("Implement type checking for %="),
            O::Assign => todo!("Implement type checking for ="),
            O::Equal => todo!("Implement type checking for =="),
            O::NotEqual => todo!("Implement type checking for !="),
            O::LessThan => todo!("Implement type checking for <"),
            O::LessThanOrEqual => todo!("Implement type checking for <="),
            O::GreaterThan => todo!("Implement type checking for >"),
            O::GreaterThanOrEqual => todo!("Implement type checking for >="),
            O::FunctionApplication => self.check_function_application(types, scope),
            O::Comma => self.check_comma(types, scope),
            O::LogicalAnd => self.check_logical_op(types, scope),
            O::LogicalOr => self.check_logical_op(types, scope),
            O::Access => panic!("ERROR: Expected ExpressionNode::Access"),
            O::Type => panic!("ERROR: Unexpected closure parameter outside of context"),
        }
    }

    fn check_function_application(
        &self,
        types: &TypeResolver,
        scope: Box<Scope>,
    ) -> (Box<Scope>, Type) {
        let (scope, left_type) = self.left.check(types, scope);
        let (scope, right_type) = self.right.check(types, scope);
        let function_type = right_type.as_function(types);

        if let Some(function_type) = function_type {
            if function_type.parameters.len() != 1 {
                println!("Type error: Right hand side of => takes more than one parameter");
            }

            if !left_type.is_assignable_to(&function_type.parameters[0], types) {
                println!("Type error: Function application argument does not match");
            }

            (scope, *function_type.return_type)
        } else {
            println!("Type error: Right hand side of => is not callable");
            (scope, Type::Error)
        }
    }

    fn check_comma(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (mut scope, first_type) = self.left.check(types, scope);
        let mut tuple_types = vec![first_type];
        let mut current = &self.right;

        loop {
            if let ExpressionNode::BinaryOp(BinaryOpExpressionNode {
                left,
                operator,
                right,
            }) = &current.value
            {
                if operator.value == BinaryOperator::Comma {
                    let (new_scope, left_type) = left.check(types, scope);
                    tuple_types.push(left_type);
                    scope = new_scope;
                    current = right;
                    continue;
                }
            }
            break;
        }

        let (scope, current_type) = current.check(types, scope);
        tuple_types.push(current_type);

        (scope, Type::Tuple(tuple_types))
    }

    fn check_logical_op(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, left_type) = self.left.check(types, scope);
        if !left_type.is_primitive(PrimitiveType::Bool, types) {
            println!(
                "Type error: Left hand side of op `{:?}` should be of type bool, found `{}`",
                self.operator.value,
                left_type.format(types),
            );
        }
        let (scope, right_type) = self.right.check(types, scope);
        if !right_type.is_primitive(PrimitiveType::Bool, types) {
            println!(
                "Type error: Right hand side of op `{:?}` should be of type bool, found `{}`",
                self.operator.value,
                right_type.format(types),
            );
        }

        (scope, Type::Primitive(PrimitiveType::Bool))
    }
}
