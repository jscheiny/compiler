use crate::{
    checker::Scope,
    parser::{FunctionNode, Identified, IdentifierNode, Node},
};

pub struct InterfaceImplementationNode {
    pub identifier: Node<IdentifierNode>,
    pub methods: Option<Vec<Node<FunctionNode>>>,
}

impl InterfaceImplementationNode {
    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        // TODO Implement type checking for interface implementations
        scope
    }
}

impl Identified for InterfaceImplementationNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
