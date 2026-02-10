use crate::parser::{BlockParseNode, ExpressionParseNode};

pub enum FunctionBodyParseNode {
    Expression(ExpressionParseNode),
    Block(BlockParseNode),
}
