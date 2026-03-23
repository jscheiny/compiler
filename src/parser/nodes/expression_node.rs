use crate::{
    checker::{Scope, ScopeType, Type},
    parser::{
        AccessExpressionNode, ArrayExpressionNode, BinaryOpExpressionNode, BlockNode,
        ClosureExpressionNode, ClosureParameterExpressionNode, DeferredAccessExpressionNode,
        FunctionCallExpressionNode, IfExpressionNode, MatchNode, NameNode, PostfixOpExpressionNode,
        PrefixOpExpressionNode, PrimitiveType, TokenSpan, TypeAccessExpressionNode,
    },
};

pub enum ExpressionNode {
    Access(AccessExpressionNode),
    Array(ArrayExpressionNode),
    BinaryOp(BinaryOpExpressionNode),
    Block(BlockNode),
    BooleanLiteral(bool),
    CharacterLiteral(String),
    Closure(ClosureExpressionNode),
    ClosureParameter(ClosureParameterExpressionNode),
    DeferredAccess(DeferredAccessExpressionNode),
    FunctionCall(FunctionCallExpressionNode),
    IfExpression(IfExpressionNode),
    IntegerLiteral(i64),
    Match(MatchNode),
    Name(NameNode),
    PostfixOp(PostfixOpExpressionNode),
    PrefixOp(PrefixOpExpressionNode),
    SelfRef(NameNode),
    SelfValue(TokenSpan),
    StringLiteral(String),
    TypeAccess(TypeAccessExpressionNode),
    Error,
}

impl ExpressionNode {
    pub fn check(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        self.check_expected(scope, None)
    }

    pub fn check_expected(
        &self,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
    ) -> (Box<Scope>, Type) {
        match self {
            Self::Access(node) => node.check(scope, expected_type),
            Self::Array(node) => node.check(scope, expected_type),
            Self::BinaryOp(node) => node.check(scope, expected_type),
            Self::Block(node) => {
                let (scope, resolved_type) = node.check(scope, expected_type);
                (scope, resolved_type.unwrap_or(Type::Void))
            }
            Self::BooleanLiteral(_) => (scope, Type::Primitive(PrimitiveType::Bool)),
            Self::CharacterLiteral(_) => (scope, Type::Primitive(PrimitiveType::Char)),
            Self::Closure(node) => node.check(scope, expected_type),
            Self::ClosureParameter(node) => node.check(scope),
            Self::DeferredAccess(node) => node.check(scope, expected_type),
            Self::FunctionCall(node) => node.check(scope, expected_type),
            Self::IfExpression(node) => node.check(scope, expected_type),
            Self::IntegerLiteral(_) => (scope, Type::Primitive(PrimitiveType::Int)),
            Self::Match(node) => node.check(scope, expected_type),
            Self::Name(node) => node.check(scope, expected_type),
            Self::PostfixOp(node) => node.check(scope),
            Self::PrefixOp(node) => node.check(scope),
            Self::SelfRef(name) => self.check_self_ref(name, scope),
            Self::SelfValue(span) => self.check_self_value(*span, scope),
            Self::StringLiteral(_) => (
                scope,
                Type::Array(Box::new(Type::Primitive(PrimitiveType::Char))),
            ),
            Self::TypeAccess(node) => node.check(scope),
            Self::Error => (scope, Type::Error),
        }
    }

    fn check_self_ref(&self, name: &NameNode, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let self_scope = scope.find_scope(|scope_type| matches!(scope_type, ScopeType::Struct(_)));
        if let Some(self_scope) = self_scope {
            let resolved_type = self_scope.get_local_value(name);
            if let Some(resolved_type) = resolved_type {
                return (scope, resolved_type);
            }
            scope.source.print_error(
                name.span,
                &format!("Could not find member `{}`", name),
                "self type does not contain a member with this name",
            );
        } else {
            scope.source.print_error(
                name.span.before(),
                "Self reference outside of struct or enum",
                "operator invalid outside of struct or enum",
            );
        }

        (scope, Type::Error)
    }

    fn check_self_value(&self, span: TokenSpan, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let mut self_index = None;
        scope.find_scope(|scope_type| match scope_type {
            ScopeType::Struct(index) => {
                self_index = Some(index);
                true
            }
            _ => false,
        });

        if let Some(index) = self_index {
            (scope, Type::Reference(index))
        } else {
            scope.source.print_error(
                span,
                "Invalid `self` outside of struct or enum",
                "`self` value only available inside of struct or enum",
            );
            (scope, Type::Error)
        }
    }
}
