use crate::parser::{ExpressionNode, MatchPatternNode, Node};

pub struct MatchCaseNode {
    pub patterns: Vec<Node<MatchPatternNode>>,
    pub if_match: Node<ExpressionNode>,
}
