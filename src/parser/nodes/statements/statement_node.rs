use crate::{
    checker::{Scope, ScopeType, Type, Types},
    lexer::Keyword,
    parser::{
        DeclarationNode, ExpressionNode, IfStatementNode, MatchNode, Node, TokenSpan,
        TypeAliasNode, WhileLoopNode,
    },
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
    TypeAlias(TypeAliasNode),
    WhileLoop(WhileLoopNode),
}

impl StatementNode {
    pub fn check(
        &self,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
        span: TokenSpan,
    ) -> (Box<Scope>, Option<Type>) {
        match self {
            Self::BlockReturn(expression) => {
                let (scope, resolved_type) = expression.check_expected(scope, expected_type);
                (scope, Some(resolved_type))
            }
            Self::Break => check_loop(Keyword::Break, span, scope),
            Self::Continue => check_loop(Keyword::Continue, span, scope),
            Self::Declaration(node) => (node.check(scope), None),
            Self::Expression(expression) => {
                // Discard the type of raw expressions
                let (scope, _) = expression.check(scope);
                (scope, None)
            }
            Self::FunctionReturn(expression) => {
                check_function_return(expression.as_ref(), span, scope)
            }
            Self::If(node) => (node.check(scope), None),
            Self::Match(node) => (node.check_statement(scope), None),
            Self::TypeAlias(node) => (node.check_statement(scope), None),
            Self::WhileLoop(node) => (node.check(scope), None),
        }
    }
}

fn check_loop(keyword: Keyword, span: TokenSpan, scope: Box<Scope>) -> (Box<Scope>, Option<Type>) {
    if !scope.within(|scope_type| matches!(scope_type, ScopeType::Loop)) {
        scope.source.print_error(
            span,
            &format!("Unexpected {keyword}"),
            &format!("{keyword} is not valid outside of a loop"),
        );
    }
    (scope, None)
}

fn check_function_return(
    expression: Option<&Node<ExpressionNode>>,
    statement_span: TokenSpan,
    mut scope: Box<Scope>,
) -> (Box<Scope>, Option<Type>) {
    let expected_type = scope.get_return_type();
    if let Some(expected_type) = expected_type {
        let (new_scope, resolved_type) = match expression {
            Some(expression) => expression.check_expected(scope, Some(&expected_type)),
            None => (scope, Type::Void),
        };
        scope = new_scope;

        if !resolved_type.is_assignable_to(&expected_type, &scope) {
            let error_span = match expression {
                Some(expression) => expression.span,
                None => statement_span,
            };
            scope.source.print_error(
                error_span,
                &format!("Function must return value of type `{}`", expected_type),
                &format!("found type: `{}`", resolved_type),
            );
        }
    } else {
        // TODO should this be a panic? I don't think this ought to occur
        scope.source.print_error(
            statement_span,
            "Unexpected return",
            "return found outside of a function",
        );
    }

    (scope, None)
}
