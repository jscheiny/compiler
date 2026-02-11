use crate::parser::{BlockNode, ExpressionNode, ParseNode};

pub struct IfStatementConditionNode {
    pub predicate: ParseNode<ExpressionNode>,
    pub body: ParseNode<BlockNode>,
}
