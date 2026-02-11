use crate::parser::{ParseNode, StatementNode};

pub struct BlockNode {
    pub statements: Vec<ParseNode<StatementNode>>,
}
