use crate::parser::{BinaryOperator, ExpressionNode, Node};

pub struct BinaryOpExpressionNode {
    pub left: Box<Node<ExpressionNode>>,
    pub operator: Node<BinaryOperator>,
    pub right: Box<Node<ExpressionNode>>,
}
