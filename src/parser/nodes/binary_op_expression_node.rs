use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{BinaryOperator, ExpressionNode, Node, PrimitiveType, get_function_type},
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
            O::Add => todo!("Implement type checking for binary op Add"),
            O::AddAssign => todo!("Implement type checking for binary op AddAssign"),
            O::Subtract => todo!("Implement type checking for binary op Subtract"),
            O::SubtractAssign => todo!("Implement type checking for binary op SubtractAssign"),
            O::Multiply => todo!("Implement type checking for binary op Multiply"),
            O::MultiplyAssign => todo!("Implement type checking for binary op MultiplyAssign"),
            O::Divide => todo!("Implement type checking for binary op Divide"),
            O::DivideAssign => todo!("Implement type checking for binary op DivideAssign"),
            O::Mod => todo!("Implement type checking for binary op Mod"),
            O::ModAssign => todo!("Implement type checking for binary op ModAssign"),
            O::Assign => todo!("Implement type checking for binary op Assign"),
            O::Equal => todo!("Implement type checking for binary op Equal"),
            O::NotEqual => todo!("Implement type checking for binary op NotEqual"),
            O::LessThan => todo!("Implement type checking for binary op LessThan"),
            O::LessThanOrEqual => todo!("Implement type checking for binary op LessThanOrEqual"),
            O::GreaterThan => todo!("Implement type checking for binary op GreaterThan"),
            O::GreaterThanOrEqual => {
                todo!("Implement type checking for binary op GreaterThanOrEqual")
            }
            O::Access => todo!("Implement type checking for binary op Access"),
            O::FunctionApplication => self.check_function_application(types, scope),
            O::Comma => todo!("Implement type checking for binary op Comma"),
            O::LogicalAnd => self.check_logical_op(types, scope),
            O::LogicalOr => self.check_logical_op(types, scope),
        }
    }

    fn check_function_application(
        &self,
        types: &TypeResolver,
        scope: Box<Scope>,
    ) -> (Box<Scope>, Type) {
        let (scope, _left_type) = self.left.check(types, scope);
        let (scope, right_type) = self.right.check(types, scope);
        let function_type = get_function_type(right_type, types);

        if let Some(function_type) = function_type {
            if function_type.parameters.len() != 1 {
                println!("Type error: Right hand side of => takes more than one parameter");
            }
            // TODO check argument type matches
            let return_type = function_type.return_type.map(|return_type| *return_type);
            // TODO properly handle void returning functions
            (scope, return_type.unwrap_or(Type::Error))
        } else {
            println!("Type error: Right hand side of => is not callable");
            (scope, Type::Error)
        }
    }

    fn check_logical_op(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, left_type) = self.left.check(types, scope);
        if !left_type.is_primitive(PrimitiveType::Bool) {
            println!(
                "Type error: Left hand side of op `{:?}` should be of type bool",
                self.operator.value
            );
        }
        let (scope, right_type) = self.right.check(types, scope);
        if !right_type.is_primitive(PrimitiveType::Bool) {
            println!(
                "Type error: Right hand side of op `{:?}` should be of type bool",
                self.operator.value
            );
        }

        (scope, Type::Primitive(PrimitiveType::Bool))
    }
}
