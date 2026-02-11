use crate::parser::{DeclarationNode, ExpressionNode, IfStatementNode, Node, WhileLoopNode};

pub enum StatementNode {
    BlockReturn(Node<ExpressionNode>),
    Break(),
    Continue(),
    Declaration(DeclarationNode),
    Expression(ExpressionNode),
    FunctionReturn(Option<Node<ExpressionNode>>),
    If(IfStatementNode),
    WhileLoop(WhileLoopNode),
}
