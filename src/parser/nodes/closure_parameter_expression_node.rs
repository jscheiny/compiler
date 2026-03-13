use crate::parser::{NameNode, Node, TypeNode};

pub struct ClosureParameterExpressionNode {
    pub name: NameNode,
    pub parameter_type: Option<Node<TypeNode>>,
}
