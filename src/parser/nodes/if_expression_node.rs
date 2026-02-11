use crate::parser::{ExpressionParseNode, ParseNode};

pub struct IfExpressionParseNode {
    pub predicate: Box<ParseNode<ExpressionParseNode>>,
    pub if_true: Box<ParseNode<ExpressionParseNode>>,
    pub if_false: Box<ParseNode<ExpressionParseNode>>,
}
