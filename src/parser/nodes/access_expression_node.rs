use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{ExpressionNode, IdentifierNode, Node},
};

pub struct AccessExpressionNode {
    pub left: Box<Node<ExpressionNode>>,
    pub field: Node<IdentifierNode>,
}

impl AccessExpressionNode {
    pub fn check(&self, _types: &TypeResolver, _scope: Box<Scope>) -> (Box<Scope>, Type) {
        todo!("Implement type checking for ExpressionNode::Access")
    }
}
