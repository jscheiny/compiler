use crate::parser::{
    DeclarationNode, ExpressionNode, IfStatementNode, ParseNode, TypeAliasNode, WhileLoopNode,
};

pub enum StatementNode {
    BlockReturn(ParseNode<ExpressionNode>),
    Break(),
    Continue(),
    Declaration(DeclarationNode),
    TypeAlias(TypeAliasNode),
    Expression(ExpressionNode),
    FunctionReturn(Option<ParseNode<ExpressionNode>>),
    If(IfStatementNode),
    WhileLoop(WhileLoopNode),
}
