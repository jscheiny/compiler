use crate::parser::{ExpressionParseNode, ParseNode, PostfixOperator};

pub struct PostfixOpExpressionParseNode {
    pub expression: Box<ParseNode<ExpressionParseNode>>,
    pub operator: ParseNode<PostfixOperator>,
}
