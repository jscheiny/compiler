use std::fmt::Debug;

use crate::{
    lexer::KeywordToken,
    parser::operator::{BinaryOperator, PostfixOperator, PrefixOperator},
};

pub type ParseResult<ParseNode> = Result<LocatedNode<ParseNode>, ()>;

#[derive(Debug)]
pub struct LocatedNode<ParseNode: Debug> {
    pub node: ParseNode,
    pub token_start_index: usize,
    pub token_end_index: usize,
}

#[derive(Debug)]
pub struct ProgramParseNode {
    pub definitions: Vec<TopLevelDefinition>,
}

#[derive(Debug)]
pub enum TopLevelDefinition {
    Record(LocatedNode<RecordDefinitionParseNode>),
    Interface(InterfaceDefinitionParseNode),
    Function(FunctionDefintionParseNode),
}

#[derive(Debug)]
pub enum TypeDefinitionParseNode {
    Primitive(KeywordToken),
    User(UserDefinedTypeParseNode),
}

#[derive(Debug)]
pub struct UserDefinedTypeParseNode {
    pub identifier: String,
    pub generic_params: Vec<TypeDefinitionParseNode>,
}

#[derive(Debug)]
pub struct InterfaceDefinitionParseNode {
    pub identifier: String,
    pub method_signatures: Vec<MethodSignatureParseNode>,
}

#[derive(Debug)]
pub struct MethodSignatureParseNode {
    pub identifier: String,
    pub parameters: Vec<ParameterParseNode>,
    pub return_type: TypeDefinitionParseNode,
}

#[derive(Debug)]
pub struct RecordDefinitionParseNode {
    pub record_type: RecordType,
    pub identifier: String,
    pub member_list: Vec<RecordMemberParseNode>,
    pub methods: Vec<MethodParseNode>,
}

#[derive(Debug)]
pub struct RecordMemberParseNode {
    pub public: bool,
    pub identifier: String,
    pub type_def: TypeDefinitionParseNode,
}

#[derive(Debug)]
pub struct MethodParseNode {
    pub public: bool,
    pub function: FunctionDefintionParseNode,
}

#[derive(Debug)]
pub struct FunctionDefintionParseNode {
    pub identifier: String,
    pub parameters: Vec<ParameterParseNode>,
    pub return_type: Option<TypeDefinitionParseNode>,
    pub body: FunctionBodyParseNode,
}

#[derive(Debug)]
pub enum FunctionBodyParseNode {
    Expression(ExpressionParseNode),
    Block(Vec<StatementParseNode>),
}

#[derive(Debug)]
pub struct ParameterParseNode {
    pub identifier: String,
    pub type_def: TypeDefinitionParseNode,
}

#[derive(Debug)]
pub enum RecordType {
    Structure,
    Tuple,
}

#[derive(Debug)]
pub enum StatementParseNode {
    Declaration(DeclarationParseNode),
    Expression(ExpressionParseNode),
    FunctionReturn(Option<ExpressionParseNode>),
    BlockReturn(ExpressionParseNode),
    Break(),
    Continue(),
    WhileLoop(WhileLoopParseNode),
    If(IfStatementParseNode),
}

#[derive(Debug)]
pub struct DeclarationParseNode {
    pub mutable: bool,
    pub identifier: String,
    pub type_def: Option<TypeDefinitionParseNode>,
    pub expression: ExpressionParseNode,
}

#[derive(Debug)]
pub struct WhileLoopParseNode {
    pub predicate: ExpressionParseNode,
    pub body: Vec<StatementParseNode>,
}

#[derive(Debug)]
pub struct IfStatementParseNode {
    pub conditions: Vec<IfStatementConditionParseNode>,
    pub else_branch: Option<Vec<StatementParseNode>>,
}

#[derive(Debug)]
pub struct IfStatementConditionParseNode {
    pub predicate: ExpressionParseNode,
    pub body: Vec<StatementParseNode>,
}

#[derive(Debug)]
pub enum ExpressionParseNode {
    PrefixOp(PrefixOpExpressionParseNode),
    BinaryOp(BinaryOpExpressionParseNode),
    PostfixOp(PostfixOpExpressionParseNode),
    StringLiteral(String),
    IntegerLiteral(i64),
    Block(Vec<StatementParseNode>),
    Identifier(String),
}

#[derive(Debug)]
pub struct PrefixOpExpressionParseNode {
    pub operator: PrefixOperator,
    pub expression: Box<ExpressionParseNode>,
}

#[derive(Debug)]
pub struct BinaryOpExpressionParseNode {
    pub left: Box<ExpressionParseNode>,
    pub operator: BinaryOperator,
    pub right: Box<ExpressionParseNode>,
}

#[derive(Debug)]
pub struct PostfixOpExpressionParseNode {
    pub expression: Box<ExpressionParseNode>,
    pub operator: PostfixOperator,
}
