use crate::parser::{ExpressionNode, ParseNode, PrefixOperator};

pub struct PrefixOpExpressionNode {
    pub operator: ParseNode<PrefixOperator>,
    pub expression: Box<ParseNode<ExpressionNode>>,
}
