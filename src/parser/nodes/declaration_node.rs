use crate::parser::{ExpressionNode, IdentifierNode, ParseNode, TypeNode};

pub struct DeclarationNode {
    pub mutable: bool,
    pub identifier: ParseNode<IdentifierNode>,
    pub type_def: Option<ParseNode<TypeNode>>,
    pub initializer: Option<ParseNode<ExpressionNode>>,
}
