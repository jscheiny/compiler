use crate::parser::{BlockNode, ExpressionNode, ParseNode};

pub struct WhileLoopNode {
    pub predicate: ParseNode<ExpressionNode>,
    pub body: ParseNode<BlockNode>,
}
