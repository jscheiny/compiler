use crate::parser::{BlockNode, ExpressionNode, Node};

pub struct IfStatementConditionNode {
    pub predicate: Node<ExpressionNode>,
    pub body: Node<BlockNode>,
}
