use crate::parser::{ParseNode, StatementParseNode};

pub struct BlockParseNode {
    pub statements: Vec<ParseNode<StatementParseNode>>,
}
