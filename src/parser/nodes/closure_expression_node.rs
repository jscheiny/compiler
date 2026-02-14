use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{ExpressionNode, IdentifierNode, Node},
};

pub struct ClosureExpressionNode {
    pub parameters: Vec<Option<Node<IdentifierNode>>>,
    pub body: Box<Node<ExpressionNode>>,
}

impl ClosureExpressionNode {
    pub fn check(&self, _types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        // TODO implement closure expression checking
        (scope, Type::Error)
    }
}
