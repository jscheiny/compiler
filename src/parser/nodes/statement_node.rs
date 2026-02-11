use crate::parser::{
    DeclarationParseNode, ExpressionParseNode, IfStatementParseNode, ParseNode, TypeAliasParseNode,
    WhileLoopParseNode,
};

pub enum StatementParseNode {
    BlockReturn(ParseNode<ExpressionParseNode>),
    Break(),
    Continue(),
    Declaration(DeclarationParseNode),
    TypeAlias(TypeAliasParseNode),
    Expression(ExpressionParseNode),
    FunctionReturn(Option<ParseNode<ExpressionParseNode>>),
    If(IfStatementParseNode),
    WhileLoop(WhileLoopParseNode),
}
