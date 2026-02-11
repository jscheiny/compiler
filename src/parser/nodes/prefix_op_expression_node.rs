use crate::parser::{ExpressionNode, Node, PrefixOperator};

pub struct PrefixOpExpressionNode {
    pub operator: Node<PrefixOperator>,
    pub expression: Box<Node<ExpressionNode>>,
}
