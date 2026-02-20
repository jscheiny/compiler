use crate::parser::{IdentifierNode, Node};

pub struct MatchPatternNode {
    pub identifier: Node<IdentifierNode>,
}
