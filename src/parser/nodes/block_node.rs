use crate::parser::{Node, StatementNode};

pub struct BlockNode {
    pub statements: Vec<Node<StatementNode>>,
}
