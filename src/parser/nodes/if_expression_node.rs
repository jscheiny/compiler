use crate::parser::{ExpressionNode, Node};

pub struct IfExpressionNode {
    pub predicate: Box<Node<ExpressionNode>>,
    pub if_true: Box<Node<ExpressionNode>>,
    pub if_false: Box<Node<ExpressionNode>>,
}
