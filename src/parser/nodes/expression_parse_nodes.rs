use std::fmt::Display;

use crate::parser::{
    BinaryOpExpressionParseNode, BlockParseNode, FunctionCallExpressionParseNode,
    IfExpressionParseNode, ParseNode, PostfixOpExpressionParseNode, PrefixOpExpressionParseNode,
    TokenSpan, Traverse,
};

pub enum ExpressionParseNode {
    PrefixOp(PrefixOpExpressionParseNode),
    BinaryOp(BinaryOpExpressionParseNode),
    PostfixOp(PostfixOpExpressionParseNode),
    FunctionCall(FunctionCallExpressionParseNode),
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
            ExpressionParseNode::FunctionCall(node) => {
                write!(f, "Call({}, (", node.function.value)?;
                for (index, arg) in node.arguments.value.iter().enumerate() {
                    write!(f, "{}", arg.value)?;
                    if index != node.arguments.value.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "))")
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
            Self::FunctionCall(node) => node.traverse(visit),
            Self::IfExpression(node) => node.traverse(visit),
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
