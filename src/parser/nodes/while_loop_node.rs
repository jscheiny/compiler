use crate::parser::{BlockNode, ExpressionNode, Node};

pub struct WhileLoopNode {
    pub predicate: Node<ExpressionNode>,
    pub body: Node<BlockNode>,
}
