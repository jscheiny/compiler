use crate::parser::{
    BinaryOperator, BlockParseNode, ParseNode, PostfixOperator, PrefixOperator, Traverse,
};

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

impl Traverse for ExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(super::TokenSpan)) {
        match self {
            Self::PrefixOp(node) => {
                node.traverse(visit);
            }
            Self::BinaryOp(node) => {
                node.traverse(visit);
            }
            Self::PostfixOp(node) => {
                node.traverse(visit);
            }
            Self::Block(node) => {
                node.traverse(visit);
            }
            Self::StringLiteral(_) | Self::IntegerLiteral(_) | Self::Identifier(_) => {}
        }
    }
}

impl Traverse for ParseNode<Box<ExpressionParseNode>> {
    fn traverse(&self, visit: &impl Fn(super::TokenSpan)) {
        visit(self.span);
        self.value.traverse(visit);
    }
}

#[derive(Debug)]
pub struct PrefixOpExpressionParseNode {
    pub operator: ParseNode<PrefixOperator>,
    pub expression: ParseNode<Box<ExpressionParseNode>>,
}

impl Traverse for PrefixOpExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(super::TokenSpan)) {
        visit(self.operator.span);
        self.expression.traverse(visit);
    }
}

#[derive(Debug)]
pub struct BinaryOpExpressionParseNode {
    pub left: ParseNode<Box<ExpressionParseNode>>,
    pub operator: ParseNode<BinaryOperator>,
    pub right: ParseNode<Box<ExpressionParseNode>>,
}

impl Traverse for BinaryOpExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(super::TokenSpan)) {
        self.left.traverse(visit);
        visit(self.operator.span);
        self.right.traverse(visit);
    }
}

#[derive(Debug)]
pub struct PostfixOpExpressionParseNode {
    pub expression: ParseNode<Box<ExpressionParseNode>>,
    pub operator: ParseNode<PostfixOperator>,
}

impl Traverse for PostfixOpExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(super::TokenSpan)) {
        self.expression.traverse(visit);
        visit(self.operator.span);
    }
}
