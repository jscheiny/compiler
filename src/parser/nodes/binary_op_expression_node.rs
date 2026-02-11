use crate::parser::{BinaryOperator, ExpressionNode, ParseNode};

pub struct BinaryOpExpressionNode {
    pub left: Box<ParseNode<ExpressionNode>>,
    pub operator: ParseNode<BinaryOperator>,
    pub right: Box<ParseNode<ExpressionNode>>,
}
