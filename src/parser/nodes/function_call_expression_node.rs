use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{ExpressionNode, Node, NodeVec},
};

pub struct FunctionCallExpressionNode {
    pub function: Box<Node<ExpressionNode>>,
    pub arguments: NodeVec<ExpressionNode>,
}

impl FunctionCallExpressionNode {
    pub fn check(&self, _types: &TypeResolver, _scope: Box<Scope>) -> (Box<Scope>, Type) {
        todo!("Implement type checking for function call")
    }
}
