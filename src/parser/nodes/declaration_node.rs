use crate::parser::{ExpressionParseNode, IdentifierParseNode, ParseNode, TypeParseNode};

pub struct DeclarationParseNode {
    pub mutable: bool,
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: Option<ParseNode<TypeParseNode>>,
    pub initializer: Option<ParseNode<ExpressionParseNode>>,
}
