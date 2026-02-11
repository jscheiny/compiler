use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{ExpressionNode, Node, PrefixOperator},
};

pub struct PrefixOpExpressionNode {
    pub operator: Node<PrefixOperator>,
    pub expression: Box<Node<ExpressionNode>>,
}

impl PrefixOpExpressionNode {
    pub fn check(&self, _types: &TypeResolver, _scope: Box<Scope>) -> (Box<Scope>, Type) {
        match *self.operator {
            PrefixOperator::Closure => todo!("Implement type checking for prefix op Closure"),
            PrefixOperator::LogicalNot => todo!("Implement type checking for prefix op LogicalNot"),
            PrefixOperator::Negative => todo!("Implement type checking for prefix op Negative"),
            PrefixOperator::SelfRef => todo!("Implement type checking for prefix op SelfRef"),
        }
    }
}
