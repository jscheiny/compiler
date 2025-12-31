use crate::parser::{ExpressionParseNode, TokenTraverser};

pub fn expression(tokens: &mut TokenTraverser) -> Result<ExpressionParseNode, ()> {
    Ok(ExpressionParseNode::Identifier(String::new()))
}
