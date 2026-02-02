use crate::parser::{
    ExpressionParseNode, IdentifierParseNode, ParseNode, TokenSpan, Traverse, TypeParseNode,
};

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
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        match self {
            Self::BlockReturn(node) => node.traverse("Statement::BlockReturn", visit),
            Self::Declaration(node) => node.traverse(visit),
            Self::Expression(node) => node.traverse(visit),
            Self::If(node) => node.traverse(visit),
            Self::WhileLoop(node) => node.traverse(visit),
            Self::FunctionReturn(node) => {
                if let Some(node) = node.as_ref() {
                    node.traverse("Statement::FunctionReturn", visit);
                }
            }
            Self::Break() | Self::Continue() => {}
        }
    }
}

pub struct BlockParseNode {
    pub statements: Vec<ParseNode<StatementParseNode>>,
}

impl Traverse for BlockParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        for statement in self.statements.iter() {
            statement.traverse("Block.statement", visit);
        }
    }
}

pub struct DeclarationParseNode {
    pub mutable: bool,
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: Option<ParseNode<TypeParseNode>>,
    pub initializer: Option<ParseNode<ExpressionParseNode>>,
}

impl Traverse for DeclarationParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("Declaration.identifier", self.identifier.span);
        if let Some(type_def) = self.type_def.as_ref() {
            type_def.traverse("Declaration.type", visit);
        }
        if let Some(expression) = self.initializer.as_ref() {
            expression.traverse("Declaration.expression", visit);
        }
    }
}

pub struct IfStatementParseNode {
    pub conditions: Vec<ParseNode<IfStatementConditionParseNode>>,
    pub else_branch: Option<ParseNode<BlockParseNode>>,
}

impl Traverse for IfStatementParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        for condition in self.conditions.iter() {
            condition.traverse("IfStatement.condition", visit);
        }
        if let Some(else_branch) = self.else_branch.as_ref() {
            else_branch.traverse("IfStatement.else", visit);
        }
    }
}

pub struct IfStatementConditionParseNode {
    pub predicate: ParseNode<ExpressionParseNode>,
    pub body: ParseNode<BlockParseNode>,
}

impl Traverse for IfStatementConditionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.predicate
            .traverse("IfStatementCondition.predicate", visit);
        self.body.traverse("IfStatementCondition.body", visit);
    }
}

pub struct WhileLoopParseNode {
    pub predicate: ParseNode<ExpressionParseNode>,
    pub body: ParseNode<BlockParseNode>,
}

impl Traverse for WhileLoopParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.predicate.traverse("WhileLoop.predicate", visit);
        self.body.traverse("WhileLoop.body", visit);
    }
}
