use crate::parser::{
    BlockParseNode, ExpressionParseNode, LocatedNode, LocatedNodeVec, TypeDefinitionParseNode,
};

#[derive(Debug)]
pub struct FunctionDefintionParseNode {
    pub identifier: LocatedNode<String>,
    pub parameters: LocatedNodeVec<ParameterParseNode>,
    pub return_type: Option<LocatedNode<TypeDefinitionParseNode>>,
    pub body: LocatedNode<FunctionBodyParseNode>,
}

#[derive(Debug)]
pub enum FunctionBodyParseNode {
    Expression(ExpressionParseNode),
    Block(BlockParseNode),
}

#[derive(Debug)]
pub struct ParameterParseNode {
    pub identifier: LocatedNode<String>,
    pub type_def: LocatedNode<TypeDefinitionParseNode>,
}
