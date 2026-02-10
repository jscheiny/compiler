use crate::parser::{BlockParseNode, ExpressionParseNode, ParseNode};

pub struct WhileLoopParseNode {
    pub predicate: ParseNode<ExpressionParseNode>,
    pub body: ParseNode<BlockParseNode>,
}
