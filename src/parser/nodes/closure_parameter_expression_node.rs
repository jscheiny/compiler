use crate::parser::{Identified, IdentifierNode, Node, TypeNode};

pub struct ClosureParameterExpressionNode {
    pub identifier: Node<IdentifierNode>,
    pub parameter_type: Option<Node<TypeNode>>,
}

impl Identified for ClosureParameterExpressionNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
