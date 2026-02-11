use crate::parser::{
    DeclarationNode, ExpressionNode, IfStatementNode, Node, TypeAliasNode, WhileLoopNode,
};

pub enum StatementNode {
    BlockReturn(Node<ExpressionNode>),
    Break(),
    Continue(),
    Declaration(DeclarationNode),
    TypeAlias(TypeAliasNode),
    Expression(ExpressionNode),
    FunctionReturn(Option<Node<ExpressionNode>>),
    If(IfStatementNode),
    WhileLoop(WhileLoopNode),
}
