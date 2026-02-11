use crate::parser::{ExpressionParseNode, ParseNode, PrefixOperator};

pub struct PrefixOpExpressionParseNode {
    pub operator: ParseNode<PrefixOperator>,
    pub expression: Box<ParseNode<ExpressionParseNode>>,
}
