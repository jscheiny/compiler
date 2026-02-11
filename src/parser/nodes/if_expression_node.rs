use crate::parser::{ExpressionNode, ParseNode};

pub struct IfExpressionNode {
    pub predicate: Box<ParseNode<ExpressionNode>>,
    pub if_true: Box<ParseNode<ExpressionNode>>,
    pub if_false: Box<ParseNode<ExpressionNode>>,
}
