use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{ExpressionNode, Node},
};

pub struct IfExpressionNode {
    pub predicate: Box<Node<ExpressionNode>>,
    pub if_true: Box<Node<ExpressionNode>>,
    pub if_false: Box<Node<ExpressionNode>>,
}

impl IfExpressionNode {
    pub fn check(&self, _types: &TypeResolver, _scope: Box<Scope>) -> (Box<Scope>, Type) {
        todo!("Implement type checking for if expression")
    }
}
