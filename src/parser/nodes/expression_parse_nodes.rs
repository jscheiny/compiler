use std::fmt::Display;

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

impl Display for ExpressionParseNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionParseNode::PrefixOp(node) => {
                write!(f, "{:?}({})", node.operator, node.expression)
            }
            ExpressionParseNode::BinaryOp(node) => {
                write!(f, "{:?}({}, {})", node.operator, node.left, node.right)
            }
            ExpressionParseNode::PostfixOp(node) => {
                write!(f, "{:?}({})", node.operator.value, node.expression.value)
            }
            ExpressionParseNode::StringLiteral(literal) => write!(f, "[{}]", literal),
            ExpressionParseNode::IntegerLiteral(literal) => write!(f, "[{}]", literal),
            ExpressionParseNode::Block(_) => write!(f, "[BLOCK]"),
            ExpressionParseNode::Identifier(identifier) => write!(f, "{}", identifier),
            ExpressionParseNode::Error => write!(f, "[ERROR]"),
        }
    }
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
    pub operator: PrefixOperator,
    pub expression: Box<ExpressionParseNode>,
}

impl Traverse for PrefixOpExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        // visit("PrefixOpExpression.operator", self.operator.span);
        self.expression.traverse(visit);
    }
}

// TODO add back locations
pub struct BinaryOpExpressionParseNode {
    pub left: Box<ExpressionParseNode>,
    pub operator: BinaryOperator,
    pub right: Box<ExpressionParseNode>,
}

impl Traverse for BinaryOpExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.left.traverse(visit);
        // visit("BinaryOpExpression.operator", self.operator.span);
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
