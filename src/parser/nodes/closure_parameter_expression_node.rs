use crate::parser::{Identified, NameNode, Node, TypeNode};

pub struct ClosureParameterExpressionNode {
    pub identifier: Node<NameNode>,
    pub parameter_type: Option<Node<TypeNode>>,
}

impl Identified for ClosureParameterExpressionNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
