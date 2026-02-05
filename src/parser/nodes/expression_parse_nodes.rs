use std::fmt::Display;

use crate::parser::{
    BinaryOperator, BlockParseNode, ParseNode, PostfixOperator, PrefixOperator, TokenSpan, Traverse,
};

pub enum ExpressionParseNode {
    PrefixOp(PrefixOpExpressionParseNode),
    BinaryOp(BinaryOpExpressionParseNode),
    PostfixOp(PostfixOpExpressionParseNode),
    IfExpression(IfExpressionParseNode),
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
                write!(f, "{:?}({})", node.operator.value, node.expression.value)
            }
            ExpressionParseNode::BinaryOp(node) => {
                write!(
                    f,
                    "{:?}({}, {})",
                    node.operator.value, node.left.value, node.right.value
                )
            }
            ExpressionParseNode::PostfixOp(node) => {
                write!(f, "{:?}({})", node.operator.value, node.expression.value)
            }
            ExpressionParseNode::IfExpression(node) => {
                write!(
                    f,
                    "If({})Then({})Else({})",
                    node.predicate.value, node.if_true.value, node.if_false.value
                )
            }
            ExpressionParseNode::StringLiteral(literal) => write!(f, "{}", literal),
            ExpressionParseNode::IntegerLiteral(literal) => write!(f, "{}", literal),
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
            Self::IfExpression(node) => node.traverse(visit),
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
    pub expression: Box<ParseNode<ExpressionParseNode>>,
}

impl Traverse for PrefixOpExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("PrefixOpExpression.operator", self.operator.span);
        self.expression
            .traverse("PrefixOpExpression.expression", visit);
    }
}

pub struct BinaryOpExpressionParseNode {
    pub left: Box<ParseNode<ExpressionParseNode>>,
    pub operator: ParseNode<BinaryOperator>,
    pub right: Box<ParseNode<ExpressionParseNode>>,
}

impl Traverse for BinaryOpExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.left.traverse("BinaryOpExpression.left", visit);
        visit("BinaryOpExpression.operator", self.operator.span);
        self.right.traverse("BinaryOpExpression.right", visit);
    }
}

pub struct PostfixOpExpressionParseNode {
    pub expression: Box<ParseNode<ExpressionParseNode>>,
    pub operator: ParseNode<PostfixOperator>,
}

impl Traverse for PostfixOpExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.expression
            .traverse("PostfixOpExpression.expression", visit);
        visit("PostfixOpExpression.operator", self.operator.span);
    }
}

pub struct IfExpressionParseNode {
    pub predicate: Box<ParseNode<ExpressionParseNode>>,
    pub if_true: Box<ParseNode<ExpressionParseNode>>,
    pub if_false: Box<ParseNode<ExpressionParseNode>>,
}

impl Traverse for IfExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.predicate.traverse("IfExpression.predicate", visit);
        self.if_true.traverse("IfExpression.if_true", visit);
        self.if_false.traverse("IfExpression.if_false", visit);
    }
}
