use crate::parser::{
    BinaryOperator, BlockParseNode, ParseNode, PostfixOperator, PrefixOperator, TokenSpan, Traverse,
};

pub enum ExpressionParseNode {
    PrefixOp(PrefixOpExpressionParseNode),
    BinaryOp(BinaryOpExpressionParseNode),
    PostfixOp(PostfixOpExpressionParseNode),
    StringLiteral(String),
    IntegerLiteral(i64),
    Block(BlockParseNode),
    Identifier(String),
    Error,
}

impl Traverse for ExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        match self {
            Self::PrefixOp(node) => node.traverse(visit),
            Self::BinaryOp(node) => node.traverse(visit),
            Self::PostfixOp(node) => node.traverse(visit),
            Self::Block(node) => node.traverse(visit),
            Self::StringLiteral(_)
            | Self::IntegerLiteral(_)
            | Self::Identifier(_)
            | Self::Error => {}
        }
    }
}

impl Traverse for ParseNode<Box<ExpressionParseNode>> {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("ExpressionParseNode.self", self.span);
        self.value.traverse(visit);
    }
}

pub struct PrefixOpExpressionParseNode {
    pub operator: ParseNode<PrefixOperator>,
    pub expression: ParseNode<Box<ExpressionParseNode>>,
}

impl Traverse for PrefixOpExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("PrefixOpExpression.operator", self.operator.span);
        self.expression.traverse(visit);
    }
}

pub struct BinaryOpExpressionParseNode {
    pub left: ParseNode<Box<ExpressionParseNode>>,
    pub operator: ParseNode<BinaryOperator>,
    pub right: ParseNode<Box<ExpressionParseNode>>,
}

impl Traverse for BinaryOpExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.left.traverse(visit);
        visit("BinaryOpExpression.operator", self.operator.span);
        self.right.traverse(visit);
    }
}

pub struct PostfixOpExpressionParseNode {
    pub expression: ParseNode<Box<ExpressionParseNode>>,
    pub operator: ParseNode<PostfixOperator>,
}

impl Traverse for PostfixOpExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.expression.traverse(visit);
        visit("PostfixOpExpression.operator", self.operator.span);
    }
}
