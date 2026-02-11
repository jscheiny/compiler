use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{ExpressionNode, Node, PostfixOperator},
};

pub struct PostfixOpExpressionNode {
    pub expression: Box<Node<ExpressionNode>>,
    pub operator: Node<PostfixOperator>,
}

impl PostfixOpExpressionNode {
    pub fn check(&self, _types: &TypeResolver, _scope: Box<Scope>) -> (Box<Scope>, Type) {
        match *self.operator {
            PostfixOperator::NullShortCircuit => {
                todo!("Implement type checking for prefix op NullShortCircuit")
            }
        }
    }
}
