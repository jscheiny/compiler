use crate::parser::{Named, NameNode, Node, TypeNode};

pub struct ClosureParameterExpressionNode {
    pub identifier: Node<NameNode>,
    pub parameter_type: Option<Node<TypeNode>>,
}

impl Named for ClosureParameterExpressionNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
