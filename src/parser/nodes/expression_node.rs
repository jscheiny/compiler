use std::fmt::Display;

use crate::parser::{
    BinaryOpExpressionNode, BlockNode, FunctionCallExpressionNode, IfExpressionNode,
    PostfixOpExpressionNode, PrefixOpExpressionNode,
};

pub enum ExpressionNode {
    PrefixOp(PrefixOpExpressionNode),
    BinaryOp(BinaryOpExpressionNode),
    PostfixOp(PostfixOpExpressionNode),
    FunctionCall(FunctionCallExpressionNode),
    IfExpression(IfExpressionNode),
    StringLiteral(String),
    IntegerLiteral(i64),
    Block(BlockNode),
    Identifier(String),
    Error,
}

impl Display for ExpressionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionNode::PrefixOp(node) => {
                write!(f, "{:?}({})", node.operator.value, node.expression.value)
            }
            ExpressionNode::BinaryOp(node) => {
                write!(
                    f,
                    "{:?}({}, {})",
                    node.operator.value, node.left.value, node.right.value
                )
            }
            ExpressionNode::PostfixOp(node) => {
                write!(f, "{:?}({})", node.operator.value, node.expression.value)
            }
            ExpressionNode::FunctionCall(node) => {
                write!(f, "Call({}, (", node.function.value)?;
                for (index, arg) in node.arguments.iter().enumerate() {
                    write!(f, "{}", arg.value)?;
                    if index != node.arguments.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "))")
            }
            ExpressionNode::IfExpression(node) => {
                write!(
                    f,
                    "If({})Then({})Else({})",
                    node.predicate.value, node.if_true.value, node.if_false.value
                )
            }
            ExpressionNode::StringLiteral(literal) => write!(f, "{}", literal),
            ExpressionNode::IntegerLiteral(literal) => write!(f, "{}", literal),
            ExpressionNode::Block(_) => write!(f, "[BLOCK]"),
            ExpressionNode::Identifier(identifier) => write!(f, "{}", identifier),
            ExpressionNode::Error => write!(f, "[ERROR]"),
        }
    }
}
