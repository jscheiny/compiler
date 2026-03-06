use crate::parser::{FunctionNode, IdentifierNode, Node};

pub struct InterfaceImplementationNode {
    pub identifier: Node<IdentifierNode>,
    pub methods: Option<Vec<Node<FunctionNode>>>,
}
