use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{BinaryOperator, ExpressionNode, Node},
};

pub struct BinaryOpExpressionNode {
    pub left: Box<Node<ExpressionNode>>,
    pub operator: Node<BinaryOperator>,
    pub right: Box<Node<ExpressionNode>>,
}

impl BinaryOpExpressionNode {
    pub fn check(&self, _types: &TypeResolver, _scope: Box<Scope>) -> (Box<Scope>, Type) {
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
            O::FunctionApplication => {
                todo!("Implement type checking for binary op FunctionApplication")
            }
            O::Comma => todo!("Implement type checking for binary op Comma"),
            O::LogicalAnd => todo!("Implement type checking for binary op LogicalAnd"),
            O::LogicalOr => todo!("Implement type checking for binary op LogicalOr"),
        }
    }
}
