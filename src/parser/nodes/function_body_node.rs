use crate::parser::{BlockNode, ExpressionNode};

pub enum FunctionBodyNode {
    Expression(ExpressionNode),
    Block(BlockNode),
}
