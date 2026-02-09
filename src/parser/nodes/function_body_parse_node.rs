use crate::parser::{BlockParseNode, ExpressionParseNode, TokenSpan, Traverse};

pub enum FunctionBodyParseNode {
    Expression(ExpressionParseNode),
    Block(BlockParseNode),
}

impl Traverse for FunctionBodyParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        match self {
            Self::Expression(node) => node.traverse(visit),
            Self::Block(node) => node.traverse(visit),
        }
    }
}
