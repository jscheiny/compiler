use crate::parser::{ExpressionParseNode, ParseNode, ParseNodeVec};

pub struct FunctionCallExpressionParseNode {
    pub function: Box<ParseNode<ExpressionParseNode>>,
    pub arguments: ParseNodeVec<ExpressionParseNode>,
}
