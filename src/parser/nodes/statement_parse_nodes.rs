use crate::parser::{ExpressionParseNode, ParseNode, TypeDefinitionParseNode};

#[derive(Debug)]
pub enum StatementParseNode {
    BlockReturn(ParseNode<ExpressionParseNode>),
    Break(),
    Continue(),
    Declaration(DeclarationParseNode),
    Expression(ExpressionParseNode),
    FunctionReturn(Option<ParseNode<ExpressionParseNode>>),
    If(IfStatementParseNode),
    WhileLoop(WhileLoopParseNode),
}

#[derive(Debug)]
pub struct BlockParseNode {
    pub statements: Vec<ParseNode<StatementParseNode>>,
}

#[derive(Debug)]
pub struct DeclarationParseNode {
    pub mutable: bool,
    pub identifier: ParseNode<String>,
    pub type_def: Option<ParseNode<TypeDefinitionParseNode>>,
    pub expression: ParseNode<ExpressionParseNode>,
}

#[derive(Debug)]
pub struct IfStatementParseNode {
    pub conditions: Vec<ParseNode<IfStatementConditionParseNode>>,
    pub else_branch: Option<ParseNode<BlockParseNode>>,
}

#[derive(Debug)]
pub struct IfStatementConditionParseNode {
    pub predicate: ParseNode<ExpressionParseNode>,
    pub body: ParseNode<BlockParseNode>,
}

#[derive(Debug)]
pub struct WhileLoopParseNode {
    pub predicate: ParseNode<ExpressionParseNode>,
    pub body: ParseNode<BlockParseNode>,
}
