use crate::parser::{ExpressionParseNode, ParseNode, TokenSpan, Traverse, TypeDefinitionParseNode};

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
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        match self {
            Self::BlockReturn(node) => node.traverse(visit),
            Self::Declaration(node) => node.traverse(visit),
            Self::Expression(node) => node.traverse(visit),
            Self::If(node) => node.traverse(visit),
            Self::WhileLoop(node) => node.traverse(visit),
            Self::FunctionReturn(node) => {
                if let Some(node) = node.as_ref() {
                    node.traverse(visit);
                }
            }
            Self::Break() | Self::Continue() => {}
        }
    }
}

#[derive(Debug)]
pub struct BlockParseNode {
    pub statements: Vec<ParseNode<StatementParseNode>>,
}

impl Traverse for BlockParseNode {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
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
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        visit(self.identifier.span);
        if let Some(type_def) = self.type_def.as_ref() {
            type_def.traverse(visit);
        }
        self.expression.traverse(visit);
    }
}

#[derive(Debug)]
pub struct IfStatementParseNode {
    pub conditions: Vec<ParseNode<IfStatementConditionParseNode>>,
    pub else_branch: Option<ParseNode<BlockParseNode>>,
}

impl Traverse for IfStatementParseNode {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
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
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
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
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        self.predicate.traverse(visit);
        self.body.traverse(visit);
    }
}
