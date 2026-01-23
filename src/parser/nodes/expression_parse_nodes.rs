use crate::parser::{BinaryOperator, BlockParseNode, LocatedNode, PostfixOperator, PrefixOperator};

#[derive(Debug)]
pub enum ExpressionParseNode {
    PrefixOp(PrefixOpExpressionParseNode),
    BinaryOp(BinaryOpExpressionParseNode),
    PostfixOp(PostfixOpExpressionParseNode),
    StringLiteral(String),
    IntegerLiteral(i64),
    Block(BlockParseNode),
    Identifier(String),
}

#[derive(Debug)]
pub struct PrefixOpExpressionParseNode {
    pub operator: LocatedNode<PrefixOperator>,
    pub expression: LocatedNode<Box<ExpressionParseNode>>,
}

#[derive(Debug)]
pub struct BinaryOpExpressionParseNode {
    pub left: LocatedNode<Box<ExpressionParseNode>>,
    pub operator: LocatedNode<BinaryOperator>,
    pub right: LocatedNode<Box<ExpressionParseNode>>,
}

#[derive(Debug)]
pub struct PostfixOpExpressionParseNode {
    pub expression: LocatedNode<Box<ExpressionParseNode>>,
    pub operator: LocatedNode<PostfixOperator>,
}
