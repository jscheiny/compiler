use crate::{
    checker::{Scope, ScopeType, Type, TypeResolver},
    lexer::KeywordToken,
    parser::{DeclarationNode, ExpressionNode, IfStatementNode, Node, WhileLoopNode},
};

pub enum StatementNode {
    BlockReturn(Node<ExpressionNode>),
    Break,
    Continue,
    Declaration(DeclarationNode),
    Expression(ExpressionNode),
    FunctionReturn(Option<Node<ExpressionNode>>),
    If(IfStatementNode),
    WhileLoop(WhileLoopNode),
}

impl StatementNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Option<Type>) {
        match self {
            Self::BlockReturn(node) => {
                // TODO check that this type matches the expected expression return type
                let (scope, resolved_type) = node.check(types, scope);
                (scope, Some(resolved_type))
            }
            Self::Break => self.check_loop(scope, KeywordToken::Break),
            Self::Continue => self.check_loop(scope, KeywordToken::Continue),
            Self::Declaration(node) => (node.check(types, scope), None),
            Self::Expression(node) => {
                // Discard the type of raw expressions
                let (scope, _) = node.check(types, scope);
                (scope, None)
            }
            Self::FunctionReturn(Some(return_type)) => {
                // TODO check return type
                let (scope, _) = return_type.check(types, scope);
                (scope, None)
            }
            Self::FunctionReturn(None) => (scope, None),
            Self::If(_node) => todo!("Implement type checking for `If`"),
            Self::WhileLoop(node) => (node.check(types, scope), None),
        }
    }

    fn check_loop(&self, scope: Box<Scope>, keyword: KeywordToken) -> (Box<Scope>, Option<Type>) {
        if !scope.within(ScopeType::Loop) {
            println!("Type error: Unexpected {} outside of loop", keyword);
        }
        (scope, None)
    }
}
