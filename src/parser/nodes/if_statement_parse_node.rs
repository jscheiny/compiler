use crate::parser::{BlockParseNode, IfStatementConditionParseNode, ParseNode};

pub struct IfStatementParseNode {
    pub conditions: Vec<ParseNode<IfStatementConditionParseNode>>,
    pub else_branch: Option<ParseNode<BlockParseNode>>,
}
