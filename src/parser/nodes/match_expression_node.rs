use crate::parser::{ExpressionNode, MatchCaseNode, Node};

pub struct MatchExpressionNode {
    pub subject: Box<Node<ExpressionNode>>,
    pub cases: Vec<Node<MatchCaseNode>>,
}
