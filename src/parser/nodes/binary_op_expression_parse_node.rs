use crate::parser::{BinaryOperator, ExpressionParseNode, ParseNode};

pub struct BinaryOpExpressionParseNode {
    pub left: Box<ParseNode<ExpressionParseNode>>,
    pub operator: ParseNode<BinaryOperator>,
    pub right: Box<ParseNode<ExpressionParseNode>>,
}
