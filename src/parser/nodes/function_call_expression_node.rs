use crate::parser::{ExpressionNode, ParseNode, ParseNodeVec};

pub struct FunctionCallExpressionNode {
    pub function: Box<ParseNode<ExpressionNode>>,
    pub arguments: ParseNodeVec<ExpressionNode>,
}
