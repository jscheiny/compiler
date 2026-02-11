use crate::parser::{BlockNode, IfStatementConditionNode, Node};

pub struct IfStatementNode {
    pub conditions: Vec<Node<IfStatementConditionNode>>,
    pub else_branch: Option<Node<BlockNode>>,
}
