use crate::parser::{InterfaceImplementationNode, MethodNode, Node};

// TODO this should probably be a single vector of enums, so that we can process things in order...
pub struct ImplementationNode {
    pub methods: Vec<Node<MethodNode>>,
    pub interface_implementations: Vec<Node<InterfaceImplementationNode>>,
}
