use crate::parser::{ExpressionNode, MatchCaseNode, Node};

pub struct MatchNode {
    pub subject: Box<Node<ExpressionNode>>,
    pub cases: Vec<Node<MatchCaseNode>>,
}
