use crate::parser::{ExpressionNode, IdentifierNode, Node, TypeNode};

pub struct DeclarationNode {
    pub mutable: bool,
    pub identifier: Node<IdentifierNode>,
    pub type_def: Option<Node<TypeNode>>,
    pub initializer: Option<Node<ExpressionNode>>,
}
