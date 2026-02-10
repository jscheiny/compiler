use crate::parser::{BlockParseNode, ExpressionParseNode, ParseNode};

pub struct IfStatementConditionParseNode {
    pub predicate: ParseNode<ExpressionParseNode>,
    pub body: ParseNode<BlockParseNode>,
}
