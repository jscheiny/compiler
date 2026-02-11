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
            Self::BlockReturn(node) => {
                // TODO check that this type matches the expected expression return type
                let (scope, resolved_type) = node.check(types, scope);
                (scope, Some(resolved_type))
            }
            Self::Break() => todo!("Implement type checking for `Break`"),
            Self::Continue() => todo!("Implement type checking for `Continue`"),
            Self::Declaration(node) => (node.check(types, scope), None),
            Self::Expression(node) => {
                // Discard the type of raw expressions
                let (scope, _) = node.check(types, scope);
                (scope, None)
            }
            Self::FunctionReturn(Some(return_type)) => {
                // TODO check return type
                let (scope, _) = return_type.check(types, scope);
                (scope, None)
            }
            Self::FunctionReturn(None) => (scope, None),
            Self::If(_node) => todo!("Implement type checking for `If`"),
            Self::WhileLoop(_node) => todo!("Implement type checking for `WhileLoop`"),
        }
    }
}
