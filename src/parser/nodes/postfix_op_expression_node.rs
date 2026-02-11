use crate::parser::{ExpressionNode, Node, PostfixOperator};

pub struct PostfixOpExpressionNode {
    pub expression: Box<Node<ExpressionNode>>,
    pub operator: Node<PostfixOperator>,
}
