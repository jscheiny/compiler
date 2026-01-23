use crate::parser::{ExpressionParseNode, ParseNode, Traverse, TypeDefinitionParseNode};

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

impl Traverse for StatementParseNode {
    fn traverse(&self, visit: &impl Fn(super::TokenSpan)) {
        match self {
            StatementParseNode::BlockReturn(node) => node.traverse(visit),
            StatementParseNode::Break() => todo!(),
            StatementParseNode::Continue() => todo!(),
            StatementParseNode::Declaration(node) => node.traverse(visit),
            StatementParseNode::Expression(node) => node.traverse(visit),
            StatementParseNode::FunctionReturn(node) => {
                node.as_ref().map(|v| v.traverse(visit));
            }
            StatementParseNode::If(node) => node.traverse(visit),
            StatementParseNode::WhileLoop(node) => node.traverse(visit),
        }
    }
}

#[derive(Debug)]
pub struct BlockParseNode {
    pub statements: Vec<ParseNode<StatementParseNode>>,
}

impl Traverse for BlockParseNode {
    fn traverse(&self, visit: &impl Fn(super::TokenSpan)) {
        for statement in self.statements.iter() {
            statement.traverse(visit);
        }
    }
}

#[derive(Debug)]
pub struct DeclarationParseNode {
    pub mutable: bool,
    pub identifier: ParseNode<String>,
    pub type_def: Option<ParseNode<TypeDefinitionParseNode>>,
    pub expression: ParseNode<ExpressionParseNode>,
}

impl Traverse for DeclarationParseNode {
    fn traverse(&self, visit: &impl Fn(super::TokenSpan)) {
        visit(self.identifier.span);
        if let Some(type_def) = self.type_def.as_ref() {
            type_def.traverse(visit);
        }
    }
}

#[derive(Debug)]
pub struct IfStatementParseNode {
    pub conditions: Vec<ParseNode<IfStatementConditionParseNode>>,
    pub else_branch: Option<ParseNode<BlockParseNode>>,
}

impl Traverse for IfStatementParseNode {
    fn traverse(&self, visit: &impl Fn(super::TokenSpan)) {
        for condition in self.conditions.iter() {
            condition.traverse(visit);
        }
        if let Some(else_branch) = self.else_branch.as_ref() {
            else_branch.traverse(visit);
        }
    }
}

#[derive(Debug)]
pub struct IfStatementConditionParseNode {
    pub predicate: ParseNode<ExpressionParseNode>,
    pub body: ParseNode<BlockParseNode>,
}

impl Traverse for IfStatementConditionParseNode {
    fn traverse(&self, visit: &impl Fn(super::TokenSpan)) {
        self.predicate.traverse(visit);
        self.body.traverse(visit);
    }
}

#[derive(Debug)]
pub struct WhileLoopParseNode {
    pub predicate: ParseNode<ExpressionParseNode>,
    pub body: ParseNode<BlockParseNode>,
}

impl Traverse for WhileLoopParseNode {
    fn traverse(&self, visit: &impl Fn(super::TokenSpan)) {
        self.predicate.traverse(visit);
        self.body.traverse(visit);
    }
}
