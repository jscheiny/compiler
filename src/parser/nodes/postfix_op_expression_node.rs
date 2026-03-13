use crate::{
    checker::{Scope, Type},
    parser::{ExpressionNode, Node, PostfixOperator},
};

pub struct PostfixOpExpressionNode {
    pub expression: Box<Node<ExpressionNode>>,
    pub operator: Node<PostfixOperator>,
}

impl PostfixOpExpressionNode {
    pub fn check(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        // TODO implement type checking for postfix ops
        (scope, Type::Error)
    }
}
