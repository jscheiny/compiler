use crate::parser::{BlockNode, IfStatementConditionNode, ParseNode};

pub struct IfStatementNode {
    pub conditions: Vec<ParseNode<IfStatementConditionNode>>,
    pub else_branch: Option<ParseNode<BlockNode>>,
}
