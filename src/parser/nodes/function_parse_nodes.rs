use crate::parser::{
    BlockParseNode, ExpressionParseNode, ParseNode, ParseNodeVec, TypeDefinitionParseNode,
};

#[derive(Debug)]
pub struct FunctionDefintionParseNode {
    pub identifier: ParseNode<String>,
    pub parameters: ParseNodeVec<ParameterParseNode>,
    pub return_type: Option<ParseNode<TypeDefinitionParseNode>>,
    pub body: ParseNode<FunctionBodyParseNode>,
}

#[derive(Debug)]
pub enum FunctionBodyParseNode {
    Expression(ExpressionParseNode),
    Block(BlockParseNode),
}

#[derive(Debug)]
pub struct ParameterParseNode {
    pub identifier: ParseNode<String>,
    pub type_def: ParseNode<TypeDefinitionParseNode>,
}
