use crate::{
    checker::{Scope, ScopeType, Type},
    lexer::Keyword,
    parser::{DeclarationNode, ExpressionNode, IfStatementNode, MatchNode, Node, WhileLoopNode},
};

pub enum StatementNode {
    BlockReturn(Node<ExpressionNode>),
    Break,
    Continue,
    Declaration(DeclarationNode),
    Expression(ExpressionNode),
    FunctionReturn(Option<Node<ExpressionNode>>),
    If(IfStatementNode),
    Match(MatchNode),
    WhileLoop(WhileLoopNode),
}

impl StatementNode {
    pub fn check(
        &self,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
    ) -> (Box<Scope>, Option<Type>) {
        match self {
            Self::BlockReturn(expression) => {
                let (scope, resolved_type) = expression.check_expected(scope, expected_type);
                (scope, Some(resolved_type))
            }
            Self::Break => check_loop(Keyword::Break, scope),
            Self::Continue => check_loop(Keyword::Continue, scope),
            Self::Declaration(node) => (node.check(scope), None),
            Self::Expression(expression) => {
                // Discard the type of raw expressions
                let (scope, _) = expression.check(scope);
                (scope, None)
            }
            Self::FunctionReturn(expression) => check_function_return(expression.as_ref(), scope),
            Self::If(node) => (node.check(scope), None),
            Self::Match(node) => (node.check_statement(scope), None),
            Self::WhileLoop(node) => (node.check(scope), None),
        }
    }
}

fn check_loop(keyword: Keyword, scope: Box<Scope>) -> (Box<Scope>, Option<Type>) {
    if !scope.within(ScopeType::Loop) {
        println!("Type error: Unexpected {} outside of loop", keyword);
    }
    (scope, None)
}

fn check_function_return(
    expression: Option<&Node<ExpressionNode>>,
    mut scope: Box<Scope>,
) -> (Box<Scope>, Option<Type>) {
    let expected_type = scope.return_type().cloned();
    if let Some(expected_type) = expected_type {
        let (new_scope, resolved_type) = match expression {
            Some(expression) => expression.check_expected(scope, Some(&expected_type)),
            None => (scope, Type::Void),
        };
        scope = new_scope;

        if !resolved_type.is_assignable_to(&expected_type, &scope.types) {
            println!(
                "Type error: Returned type `{}` is not assignable to expected return type of `{}`",
                resolved_type.format(&scope.types),
                expected_type.format(&scope.types)
            );
        }
    } else {
        println!("Type error: Return found in non function context");
    }

    (scope, None)
}
