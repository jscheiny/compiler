use crate::parser::{NameNode, Named, Node, TypeNode};

pub struct ClosureParameterExpressionNode {
    pub identifier: Node<NameNode>,
    pub parameter_type: Option<Node<TypeNode>>,
}

impl Named for ClosureParameterExpressionNode {
    fn name(&self) -> &String {
        self.identifier.name()
    }
}
