use crate::parser::{BinaryOperator, BlockParseNode, ParseNode, PostfixOperator, PrefixOperator};

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
    pub operator: ParseNode<PrefixOperator>,
    pub expression: ParseNode<Box<ExpressionParseNode>>,
}

#[derive(Debug)]
pub struct BinaryOpExpressionParseNode {
    pub left: ParseNode<Box<ExpressionParseNode>>,
    pub operator: ParseNode<BinaryOperator>,
    pub right: ParseNode<Box<ExpressionParseNode>>,
}

#[derive(Debug)]
pub struct PostfixOpExpressionParseNode {
    pub expression: ParseNode<Box<ExpressionParseNode>>,
    pub operator: ParseNode<PostfixOperator>,
}
