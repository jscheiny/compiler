// use std::cell::OnceCell;

use crate::{
    checker::Scope,
    parser::{FunctionSignatureNode, IdentifierNode, Node, NodeVec},
};

pub struct InterfaceNode {
    pub identifier: Node<IdentifierNode>,
    method_signatures: NodeVec<FunctionSignatureNode>,
    // resolved_type: OnceCell<i64>,
}

impl InterfaceNode {
    pub fn new(
        identifier: Node<IdentifierNode>,
        method_signatures: NodeVec<FunctionSignatureNode>,
    ) -> Self {
        Self {
            identifier,
            method_signatures,
            // resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        todo!("Implement type checking for interfaces")
    }
}
