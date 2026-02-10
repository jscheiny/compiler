use crate::parser::{
    DeclarationParseNode, ExpressionParseNode, IfStatementParseNode, ParseNode, TokenSpan,
    Traverse, TypeAliasParseNode, WhileLoopParseNode,
};

pub enum StatementParseNode {
    BlockReturn(ParseNode<ExpressionParseNode>),
    Break(),
    Continue(),
    Declaration(DeclarationParseNode),
    TypeAlias(TypeAliasParseNode),
    Expression(ExpressionParseNode),
    FunctionReturn(Option<ParseNode<ExpressionParseNode>>),
    If(IfStatementParseNode),
    WhileLoop(WhileLoopParseNode),
}

impl Traverse for StatementParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        match self {
            Self::BlockReturn(node) => node.traverse("Statement::BlockReturn", visit),
            Self::Declaration(node) => node.traverse(visit),
            Self::TypeAlias(node) => node.traverse(visit),
            Self::Expression(node) => node.traverse(visit),
            Self::If(node) => node.traverse(visit),
            Self::WhileLoop(node) => node.traverse(visit),
            Self::FunctionReturn(node) => {
                if let Some(node) = node.as_ref() {
                    node.traverse("Statement::FunctionReturn", visit);
                }
            }
            Self::Break() | Self::Continue() => {}
        }
    }
}
