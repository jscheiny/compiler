use crate::parser::{IdentifierNode, Node, TypeNode};

pub struct ClosureParameterExpressionNode {
    pub identifier: Node<IdentifierNode>,
    pub parameter_type: Option<Node<TypeNode>>,
}
