use crate::parser::{ExpressionNode, ParseNode, PostfixOperator};

pub struct PostfixOpExpressionNode {
    pub expression: Box<ParseNode<ExpressionNode>>,
    pub operator: ParseNode<PostfixOperator>,
}
