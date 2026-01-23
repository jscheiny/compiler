use crate::parser::{ExpressionParseNode, LocatedNode, TypeDefinitionParseNode};

#[derive(Debug)]
pub enum StatementParseNode {
    BlockReturn(LocatedNode<ExpressionParseNode>),
    Break(),
    Continue(),
    Declaration(DeclarationParseNode),
    Expression(ExpressionParseNode),
    FunctionReturn(Option<LocatedNode<ExpressionParseNode>>),
    If(IfStatementParseNode),
    WhileLoop(WhileLoopParseNode),
}

#[derive(Debug)]
pub struct BlockParseNode {
    pub statements: Vec<LocatedNode<StatementParseNode>>,
}

#[derive(Debug)]
pub struct DeclarationParseNode {
    pub mutable: bool,
    pub identifier: LocatedNode<String>,
    pub type_def: Option<LocatedNode<TypeDefinitionParseNode>>,
    pub expression: LocatedNode<ExpressionParseNode>,
}

#[derive(Debug)]
pub struct IfStatementParseNode {
    pub conditions: Vec<LocatedNode<IfStatementConditionParseNode>>,
    pub else_branch: Option<LocatedNode<BlockParseNode>>,
}

#[derive(Debug)]
pub struct IfStatementConditionParseNode {
    pub predicate: LocatedNode<ExpressionParseNode>,
    pub body: LocatedNode<BlockParseNode>,
}

#[derive(Debug)]
pub struct WhileLoopParseNode {
    pub predicate: LocatedNode<ExpressionParseNode>,
    pub body: LocatedNode<BlockParseNode>,
}
