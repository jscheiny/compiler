use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{DeclarationNode, ExpressionNode, IfStatementNode, Node, WhileLoopNode},
};

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

impl StatementNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Option<Type>) {
        match self {
            Self::BlockReturn(_node) => todo!("Implement type checking for `BlockReturn`"),
            Self::Break() => todo!("Implement type checking for `Break`"),
            Self::Continue() => todo!("Implement type checking for `Continue`"),
            Self::Declaration(node) => (node.check(types, scope), None),
            Self::Expression(_node) => todo!("Implement type checking for `Expression`"),
            Self::FunctionReturn(_node) => todo!("Implement type checking for `FunctionReturn`"),
            Self::If(_node) => todo!("Implement type checking for `If`"),
            Self::WhileLoop(_node) => todo!("Implement type checking for `WhileLoop`"),
        }
    }
}
